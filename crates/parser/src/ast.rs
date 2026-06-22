//! SQL 抽象语法树
use sqlrustgo_common::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    CreateTable {
        name: String,
        columns: Vec<ColumnDef>,
    },
    DropTable {
        name: String,
    },
    Insert {
        table: String,
        values: Vec<Value>,
    },
    Select {
        table: String,
        where_clause: Option<Expr>,
    },
    Update {
        table: String,
        assignments: Vec<(String, Value)>,
        where_clause: Option<Expr>,
    },
    Delete {
        table: String,
        where_clause: Option<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Text,
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Column(String),
    Binary { left: Box<Expr>, op: BinOp, right: Box<Expr> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}
