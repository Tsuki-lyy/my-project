//! 物理计划
use sqlrustgo_common::Value;
use sqlrustgo_parser::Expr;

use crate::logical::LogicalPlan;

#[derive(Debug, Clone)]
pub enum PhysicalPlan {
    CreateTable { name: String, columns: Vec<(String, String)> },
    DropTable { name: String },
    Insert { table: String, values: Vec<Value> },
    Project { input: Box<LogicalPlan> },
    Update { table: String, assignments: Vec<(String, Value)>, filter: Option<Expr> },
    Delete { table: String, filter: Option<Expr> },
}
