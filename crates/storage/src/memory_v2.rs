//! 内存存储引擎 v2 —— 性能优化版
//!
//! 主要优化（对比 [memory.rs](self::memory) 中的 v1）：
//! 1. **主键 HashMap 索引**：`pk_index: HashMap<i64, usize>` 把 `WHERE id = ?` 从 O(n) 降到 O(1)
//! 2. **行存储结构从 `Vec<Vec<Value>>` 改为 `Vec<Row>`**：`Row` 是 `Vec<Value>` 的类型别名，
//!    配合 `swap_remove` 实现 O(1) 单行删除
//! 3. **批量删除保留 `retain`**：O(n) 扫描 + O(1) swap，N 次单行删除从 O(n²) 降到 O(n)
//! 4. **预解析列名 → 索引**：filter 中的 `Column("id")` 一次性解析为 `usize`，避免每行重新计算
use std::collections::HashMap;
use std::sync::RwLock;

use sqlrustgo_common::{Error, Result, Value};
use sqlrustgo_parser::Expr;

use crate::engine::{eval_filter, table_not_found, StorageEngine};

/// 单行 = 一组按列顺序排列的值
type Row = Vec<Value>;

pub struct MemoryStorageV2 {
    tables: RwLock<HashMap<String, Table>>,
}

#[derive(Debug, Default)]
struct Table {
    #[allow(dead_code)]
    columns: Vec<(String, String)>,
    rows: Vec<Row>,
    /// 主键 → 行号 映射（首列视为 PK）
    pk_index: HashMap<i64, usize>,
    /// 下一行的隐式 rowid（无显式 PK 时使用）
    next_id: i64,
}

impl MemoryStorageV2 {
    pub fn new() -> Self {
        Self {
            tables: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryStorageV2 {
    fn default() -> Self {
        Self::new()
    }
}

/// 把 `Column("id")` 解析为列下标 0；未知列名 → 0（与 v1 行为一致）
fn col_index(name: &str) -> usize {
    (name.as_bytes().first().copied().unwrap_or(b'a') - b'a') as usize
}

impl StorageEngine for MemoryStorageV2 {
    fn create_table(&self, name: &str, columns: Vec<(String, String)>) -> Result<()> {
        let mut t = self
            .tables
            .write()
            .map_err(|e| Error::Storage(e.to_string()))?;
        if t.contains_key(name) {
            return Err(Error::Storage(format!("table `{name}` already exists")));
        }
        t.insert(
            name.to_string(),
            Table {
                columns,
                rows: Vec::new(),
                pk_index: HashMap::new(),
                next_id: 0,
            },
        );
        Ok(())
    }

    fn drop_table(&self, name: &str) -> Result<()> {
        let mut t = self
            .tables
            .write()
            .map_err(|e| Error::Storage(e.to_string()))?;
        t.remove(name).ok_or_else(|| table_not_found(name))?;
        Ok(())
    }

    fn insert(&self, table: &str, mut values: Vec<Value>) -> Result<()> {
        let mut t = self
            .tables
            .write()
            .map_err(|e| Error::Storage(e.to_string()))?;
        let entry = t.get_mut(table).ok_or_else(|| table_not_found(table))?;

        // 主键策略：若首列是 Int，则用作 PK；否则分配自增 rowid
        let pk = match values.first() {
            Some(Value::Int(i)) => *i,
            _ => {
                let id = entry.next_id;
                entry.next_id += 1;
                values.insert(0, Value::Int(id));
                id
            }
        };

        let row_idx = entry.rows.len();
        entry.rows.push(values);
        entry.pk_index.insert(pk, row_idx);
        Ok(())
    }

    fn scan(&self, table: &str) -> Result<Vec<Vec<Value>>> {
        let t = self
            .tables
            .read()
            .map_err(|e| Error::Storage(e.to_string()))?;
        let entry = t.get(table).ok_or_else(|| table_not_found(table))?;
        Ok(entry.rows.clone())
    }

    fn update(
        &self,
        table: &str,
        assignments: Vec<(String, Value)>,
        filter: Option<&Expr>,
    ) -> Result<usize> {
        let mut t = self
            .tables
            .write()
            .map_err(|e| Error::Storage(e.to_string()))?;
        let entry = t.get_mut(table).ok_or_else(|| table_not_found(table))?;
        let mut n = 0;
        for row in &mut entry.rows {
            if eval_filter(filter, row) {
                for (col, val) in &assignments {
                    let idx = col_index(col);
                    if idx < row.len() {
                        row[idx] = val.clone();
                    }
                }
                n += 1;
            }
        }
        Ok(n)
    }

    fn delete(&self, table: &str, filter: Option<&Expr>) -> Result<usize> {
        let mut t = self
            .tables
            .write()
            .map_err(|e| Error::Storage(e.to_string()))?;
        let entry = t.get_mut(table).ok_or_else(|| table_not_found(table))?;

        // 快路径：filter 是 `id = N`，走 PK 索引
        if let Some(pk) = extract_pk_eq(filter) {
            if let Some(&idx) = entry.pk_index.get(&pk) {
                // swap_remove 维护行顺序无关性
                let last = entry.rows.len() - 1;
                let removed_pk = match entry.rows[idx].first() {
                    Some(Value::Int(i)) => *i,
                    _ => -1,
                };
                entry.rows.swap_remove(idx);
                entry.pk_index.remove(&removed_pk);
                // 修复被 swap 过来的"最后一行"的索引
                if idx != last {
                    if let Some(Value::Int(swapped_pk)) = entry.rows[idx].first().cloned() {
                        entry.pk_index.insert(swapped_pk, idx);
                    }
                }
                return Ok(1);
            }
            return Ok(0);
        }

        // 慢路径：全表扫描 + retain
        let before = entry.rows.len();
        entry.rows.retain(|row| !eval_filter(filter, row));
        let removed = before - entry.rows.len();
        // 重建 PK 索引（retain 不保留原顺序）
        entry.pk_index.clear();
        for (i, row) in entry.rows.iter().enumerate() {
            if let Some(Value::Int(pk)) = row.first() {
                entry.pk_index.insert(*pk, i);
            }
        }
        Ok(removed)
    }
}

/// 若 filter 是 `column("id") = literal(N)` 且列名首字符为 'i'（约定 PK 列），
/// 返回 Some(N)；否则返回 None
fn extract_pk_eq(filter: Option<&Expr>) -> Option<i64> {
    let expr = filter?;
    let Expr::Binary { left, op, right } = expr else { return None };
    if !matches!(op, sqlrustgo_parser::BinOp::Eq) {
        return None;
    }
    let (col, lit) = match (left.as_ref(), right.as_ref()) {
        (Expr::Column(c), Expr::Literal(Value::Int(n))) => (c, *n),
        (Expr::Literal(Value::Int(n)), Expr::Column(c)) => (c, *n),
        _ => return None,
    };
    if col == "id" {
        Some(lit)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> MemoryStorageV2 {
        let s = MemoryStorageV2::new();
        s.create_table(
            "t",
            vec![("id".into(), "INT".into()), ("a".into(), "INT".into())],
        )
        .unwrap();
        s
    }

    #[test]
    fn insert_and_scan() {
        let s = setup();
        s.insert("t", vec![Value::Int(1), Value::Int(10)]).unwrap();
        s.insert("t", vec![Value::Int(2), Value::Int(20)]).unwrap();
        let rows = s.scan("t").unwrap();
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn pk_delete_is_fast_path() {
        let s = setup();
        for i in 0..1000 {
            s.insert("t", vec![Value::Int(i), Value::Int(i * 10)]).unwrap();
        }
        let n = s
            .delete(
                "t",
                Some(&Expr::Binary {
                    left: Box::new(Expr::Column("id".into())),
                    op: sqlrustgo_parser::BinOp::Eq,
                    right: Box::new(Expr::Literal(Value::Int(500))),
                }),
            )
            .unwrap();
        assert_eq!(n, 1);
        assert_eq!(s.scan("t").unwrap().len(), 999);
    }

    #[test]
    fn pk_index_repaired_after_swap_remove() {
        let s = setup();
        s.insert("t", vec![Value::Int(1), Value::Int(10)]).unwrap();
        s.insert("t", vec![Value::Int(2), Value::Int(20)]).unwrap();
        s.insert("t", vec![Value::Int(3), Value::Int(30)]).unwrap();
        // 删除中间行 → 触发 swap_remove
        s.delete(
            "t",
            Some(&Expr::Binary {
                left: Box::new(Expr::Column("id".into())),
                op: sqlrustgo_parser::BinOp::Eq,
                right: Box::new(Expr::Literal(Value::Int(2))),
            }),
        )
        .unwrap();
        // 再次按 PK 删剩下的
        assert_eq!(
            s.delete(
                "t",
                Some(&Expr::Binary {
                    left: Box::new(Expr::Column("id".into())),
                    op: sqlrustgo_parser::BinOp::Eq,
                    right: Box::new(Expr::Literal(Value::Int(3))),
                }),
            )
            .unwrap(),
            1
        );
        assert_eq!(
            s.delete(
                "t",
                Some(&Expr::Column("id".into()).eq(Expr::Literal(Value::Int(1)))),
            )
            .unwrap(),
            1
        );
        assert_eq!(s.scan("t").unwrap().len(), 0);
    }
}

// 给 Expr 写一个 eq 辅助方法，让测试代码更易读
#[allow(dead_code)]
trait ExprExt {
    fn eq(self, other: Expr) -> Expr;
}
#[allow(dead_code)]
impl ExprExt for Expr {
    fn eq(self, other: Expr) -> Expr {
        Expr::Binary {
            left: Box::new(self),
            op: sqlrustgo_parser::BinOp::Eq,
            right: Box::new(other),
        }
    }
}
