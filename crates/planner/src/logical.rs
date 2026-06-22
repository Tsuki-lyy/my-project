//! 逻辑计划与规划器
use sqlrustgo_common::Result;
use sqlrustgo_parser::Statement;

use crate::physical::PhysicalPlan;

#[derive(Debug, Clone)]
pub enum LogicalPlan {
    CreateTable {
        name: String,
        columns: Vec<(String, String)>,
    },
    DropTable {
        name: String,
    },
    Insert {
        table: String,
        values: Vec<sqlrustgo_common::Value>,
    },
    Scan {
        table: String,
        filter: Option<sqlrustgo_parser::Expr>,
    },
    Update {
        table: String,
        assignments: Vec<(String, sqlrustgo_common::Value)>,
        filter: Option<sqlrustgo_parser::Expr>,
    },
    Delete {
        table: String,
        filter: Option<sqlrustgo_parser::Expr>,
    },
    Project {
        input: Box<LogicalPlan>,
    },
    Filter {
        input: Box<LogicalPlan>,
        predicate: sqlrustgo_parser::Expr,
    },
    Limit {
        input: Box<LogicalPlan>,
        limit: u64,
        offset: u64,
    },
    OrderBy {
        input: Box<LogicalPlan>,
        column: String,
        desc: bool,
    },
}

pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        Self
    }

    pub fn plan(&self, stmt: Statement) -> Result<PhysicalPlan> {
        match stmt {
            Statement::CreateTable { name, columns } => {
                let cols = columns
                    .into_iter()
                    .map(|c| (c.name, format!("{:?}", c.data_type)))
                    .collect();
                Ok(PhysicalPlan::CreateTable {
                    name,
                    columns: cols,
                })
            }
            Statement::DropTable { name } => Ok(PhysicalPlan::DropTable { name }),
            Statement::Insert { table, values } => Ok(PhysicalPlan::Insert { table, values }),
            Statement::Select {
                table,
                where_clause,
                limit,
                offset,
                order_by,
            } => {
                let mut plan = LogicalPlan::Scan {
                    table,
                    filter: where_clause,
                };
                if let Some((col, desc)) = order_by {
                    plan = LogicalPlan::OrderBy {
                        input: Box::new(plan),
                        column: col,
                        desc,
                    };
                }
                if limit.is_some() || offset.is_some() {
                    plan = LogicalPlan::Limit {
                        input: Box::new(plan),
                        limit: limit.unwrap_or(0),
                        offset: offset.unwrap_or(0),
                    };
                }
                Ok(PhysicalPlan::Project {
                    input: Box::new(plan),
                })
            }
            Statement::Update {
                table,
                assignments,
                where_clause,
            } => Ok(PhysicalPlan::Update {
                table,
                assignments,
                filter: where_clause,
            }),
            Statement::Delete {
                table,
                where_clause,
            } => Ok(PhysicalPlan::Delete {
                table,
                filter: where_clause,
            }),
        }
    }
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}
