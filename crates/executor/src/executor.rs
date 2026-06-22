//! 执行器
use sqlrustgo_catalog::Catalog;
use sqlrustgo_common::{Error, Result, Value};
use sqlrustgo_planner::{LogicalPlan, PhysicalPlan};
use sqlrustgo_storage::StorageEngine;

/// 简化的执行器 - 内部使用 Box 以避免复杂生命周期
pub struct Executor {
    storage: Box<dyn StorageEngine>,
    catalog: Box<Catalog>,
}

impl Executor {
    pub fn new(storage: Box<dyn StorageEngine>, catalog: Box<Catalog>) -> Self {
        Self { storage, catalog }
    }

    pub fn execute(&self, plan: &PhysicalPlan) -> Result<Vec<Vec<Value>>> {
        match plan {
            PhysicalPlan::CreateTable { name, columns } => {
                self.catalog.create_table(name, columns.clone());
                self.storage.create_table(name, columns.clone())?;
                Ok(vec![])
            }
            PhysicalPlan::DropTable { name } => {
                self.catalog.drop_table(name);
                self.storage.drop_table(name)?;
                Ok(vec![])
            }
            PhysicalPlan::Insert { table, values } => {
                self.storage.insert(table, values.clone())?;
                Ok(vec![])
            }
            PhysicalPlan::Update {
                table,
                assignments,
                filter,
            } => {
                let count = self
                    .storage
                    .update(table, assignments.clone(), filter.as_ref())?;
                Ok(vec![vec![Value::Int(count as i64)]])
            }
            PhysicalPlan::Delete { table, filter } => {
                let count = self.storage.delete(table, filter.as_ref())?;
                Ok(vec![vec![Value::Int(count as i64)]])
            }
            PhysicalPlan::Project { input } => self.execute_logical(input),
        }
    }

    fn execute_logical(&self, plan: &LogicalPlan) -> Result<Vec<Vec<Value>>> {
        match plan {
            LogicalPlan::Scan { table, filter } => {
                let mut rows = self.storage.scan(table)?;
                if let Some(expr) = filter {
                    rows.retain(|row| eval_expr(expr, row));
                }
                Ok(rows)
            }
            LogicalPlan::Project { input } => self.execute_logical(input),
            LogicalPlan::Filter { input, predicate } => {
                let rows = self.execute_logical(input)?;
                Ok(rows
                    .into_iter()
                    .filter(|row| eval_expr(predicate, row))
                    .collect())
            }
            LogicalPlan::Limit {
                input,
                limit,
                offset,
            } => {
                let rows = self.execute_logical(input)?;
                let start = (*offset as usize).min(rows.len());
                let end = if *limit == 0 {
                    rows.len()
                } else {
                    (start + *limit as usize).min(rows.len())
                };
                Ok(rows[start..end].to_vec())
            }
            LogicalPlan::OrderBy {
                input,
                column,
                desc,
            } => {
                let mut rows = self.execute_logical(input)?;
                let idx = column.as_bytes();
                let col_idx = (idx.first().copied().unwrap_or(b'a') - b'a') as usize;
                rows.sort_by(|a, b| {
                    let av = a.get(col_idx).cloned().unwrap_or(Value::Null);
                    let bv = b.get(col_idx).cloned().unwrap_or(Value::Null);
                    let ord = av.partial_cmp(&bv).unwrap_or(std::cmp::Ordering::Equal);
                    if *desc { ord.reverse() } else { ord }
                });
                Ok(rows)
            }
            _ => Err(Error::Execution("unsupported logical plan".into())),
        }
    }
}

fn eval_expr(expr: &sqlrustgo_parser::Expr, row: &[Value]) -> bool {
    use sqlrustgo_parser::{BinOp, Expr};
    match expr {
        Expr::Literal(v) => match v {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            _ => true,
        },
        Expr::Column(_) => true,
        Expr::Binary { left, op, right } => {
            let l = eval_scalar(left, row);
            let r = eval_scalar(right, row);
            match op {
                BinOp::Eq => l == r,
                BinOp::Ne => l != r,
                BinOp::Lt => l < r,
                BinOp::Le => l <= r,
                BinOp::Gt => l > r,
                BinOp::Ge => l >= r,
                _ => false,
            }
        }
    }
}

fn eval_scalar(expr: &sqlrustgo_parser::Expr, row: &[Value]) -> Value {
    use sqlrustgo_parser::Expr;
    match expr {
        Expr::Literal(v) => v.clone(),
        Expr::Column(name) => {
            // 简化：列名按字典序位置解析
            let idx = (name.bytes().next().unwrap_or(b'a') - b'a') as usize;
            row.get(idx).cloned().unwrap_or(Value::Null)
        }
        Expr::Binary { .. } => Value::Null,
    }
}
