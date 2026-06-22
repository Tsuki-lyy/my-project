//! 内存存储引擎（HashMap 实现）
use std::collections::HashMap;
use std::sync::RwLock;

use sqlrustgo_common::{Error, Result, Value};
use sqlrustgo_parser::Expr;

use crate::engine::{eval_filter, table_not_found, StorageEngine};

pub struct MemoryStorage {
    tables: RwLock<HashMap<String, Table>>,
}

#[derive(Debug, Default)]
struct Table {
    #[allow(dead_code)]
    columns: Vec<(String, String)>,
    rows: Vec<Vec<Value>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            tables: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageEngine for MemoryStorage {
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

    fn insert(&self, table: &str, values: Vec<Value>) -> Result<()> {
        let mut t = self
            .tables
            .write()
            .map_err(|e| Error::Storage(e.to_string()))?;
        let entry = t.get_mut(table).ok_or_else(|| table_not_found(table))?;
        entry.rows.push(values);
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
                    let idx = (col.bytes().next().unwrap_or(b'a') - b'a') as usize;
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
        let before = entry.rows.len();
        entry.rows.retain(|row| !eval_filter(filter, row));
        Ok(before - entry.rows.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlrustgo_parser::Expr;

    fn setup() -> MemoryStorage {
        let s = MemoryStorage::new();
        s.create_table(
            "t",
            vec![("a".into(), "INT".into()), ("b".into(), "INT".into())],
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
    fn delete_with_filter() {
        let s = setup();
        s.insert("t", vec![Value::Int(1), Value::Int(10)]).unwrap();
        s.insert("t", vec![Value::Int(2), Value::Int(20)]).unwrap();
        // 简化：列名首字母 -> 位置
        let filter = Expr::Binary {
            left: Box::new(Expr::Column("a".into())),
            op: sqlrustgo_parser::BinOp::Eq,
            right: Box::new(Expr::Literal(Value::Int(1))),
        };
        let n = s.delete("t", Some(&filter)).unwrap();
        assert_eq!(n, 1);
        assert_eq!(s.scan("t").unwrap().len(), 1);
    }
}
