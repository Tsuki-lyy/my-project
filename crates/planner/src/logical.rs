//! 逻辑计划与规划器
use sqlrustgo_common::Result;
use sqlrustgo_parser::Statement;

use crate::physical::PhysicalPlan;

#[derive(Debug, Clone)]
pub enum LogicalPlan {
    CreateTable { name: String, columns: Vec<(String, String)> },
    DropTable { name: String },
    Insert { table: String, values: Vec<sqlrustgo_common::Value> },
    Scan { table: String, filter: Option<sqlrustgo_parser::Expr> },
    Update { table: String, assignments: Vec<(String, sqlrustgo_common::Value)>, filter: Option<sqlrustgo_parser::Expr> },
    Delete { table: String, filter: Option<sqlrustgo_parser::Expr> },
    Project { input: Box<LogicalPlan> },
    Filter { input: Box<LogicalPlan>, predicate: sqlrustgo_parser::Expr },
}

pub struct Planner;

impl Planner {
    pub fn new() -> Self { Self }

    pub fn plan(&self, stmt: Statement) -> Result<PhysicalPlan> {
        match stmt {
            Statement::CreateTable { name, columns } => {
                let cols = columns
                    .into_iter()
                    .map(|c| (c.name, format!("{:?}", c.data_type)))
                    .collect();
                Ok(PhysicalPlan::CreateTable { name, columns: cols })
            }
            Statement::DropTable { name } => Ok(PhysicalPlan::DropTable { name }),
            Statement::Insert { table, values } => Ok(PhysicalPlan::Insert { table, values }),
            Statement::Select { table, where_clause } => {
                let plan = LogicalPlan::Scan { table, filter: where_clause };
                Ok(PhysicalPlan::Project {
                    input: Box::new(plan),
                })
            }
            Statement::Update { table, assignments, where_clause } => {
                Ok(PhysicalPlan::Update {
                    table,
                    assignments,
                    filter: where_clause,
                })
            }
            Statement::Delete { table, where_clause } => {
                Ok(PhysicalPlan::Delete { table, filter: where_clause })
            }
        }
    }
}

impl Default for Planner {
    fn default() -> Self { Self::new() }
}
