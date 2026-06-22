//! 存储引擎抽象接口
use sqlrustgo_common::{Error, Result, Value};
use sqlrustgo_parser::Expr;

pub trait StorageEngine: Send + Sync {
    fn create_table(&self, name: &str, columns: Vec<(String, String)>) -> Result<()>;
    fn drop_table(&self, name: &str) -> Result<()>;
    fn insert(&self, table: &str, values: Vec<Value>) -> Result<()>;
    fn scan(&self, table: &str) -> Result<Vec<Vec<Value>>>;
    fn update(
        &self,
        table: &str,
        assignments: Vec<(String, Value)>,
        filter: Option<&Expr>,
    ) -> Result<usize>;
    fn delete(&self, table: &str, filter: Option<&Expr>) -> Result<usize>;
}

/// 简单的真值评估
pub fn eval_filter(filter: Option<&Expr>, row: &[Value]) -> bool {
    let Some(expr) = filter else { return true };
    eval(expr, row)
}

fn eval(expr: &Expr, row: &[Value]) -> bool {
    match expr {
        Expr::Literal(v) => match v {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            _ => true,
        },
        Expr::Column(name) => {
            let idx = (name.bytes().next().unwrap_or(b'a') - b'a') as usize;
            match row.get(idx) {
                Some(Value::Bool(b)) => *b,
                Some(Value::Int(i)) => *i != 0,
                Some(Value::Null) => false,
                _ => true,
            }
        }
        Expr::Binary { left, op, right } => {
            let l = scalar(left, row);
            let r = scalar(right, row);
            use sqlrustgo_parser::BinOp::*;
            match op {
                Eq => l == r,
                Ne => l != r,
                Lt => l < r,
                Le => l <= r,
                Gt => l > r,
                Ge => l >= r,
                And => eval(left, row) && eval(right, row),
                Or => eval(left, row) || eval(right, row),
            }
        }
    }
}

fn scalar(expr: &Expr, row: &[Value]) -> Value {
    match expr {
        Expr::Literal(v) => v.clone(),
        Expr::Column(name) => {
            let idx = (name.bytes().next().unwrap_or(b'a') - b'a') as usize;
            row.get(idx).cloned().unwrap_or(Value::Null)
        }
        Expr::Binary { .. } => Value::Null,
    }
}

/// 当表未实现时给上层清晰的错误
pub fn table_not_found(name: &str) -> Error {
    Error::Storage(format!("table `{name}` not found"))
}
