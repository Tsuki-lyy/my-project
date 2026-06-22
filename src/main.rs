//! sqlrustgo - SQLRustGo 数据库驱动入口
//!
//! AI增强的软件工程课程实验项目
//! 学号: 202442020106

use sqlrustgo_catalog::Catalog;
use sqlrustgo_common::Result;
use sqlrustgo_executor::Executor;
use sqlrustgo_parser::Parser;
use sqlrustgo_planner::Planner;
use sqlrustgo_storage::{MemoryStorage, StorageEngine};

/// 简化的数据库实例
pub struct Database {
    executor: Box<Executor>,
}

impl Database {
    /// 创建一个内存数据库
    pub fn new_in_memory() -> Result<Self> {
        let storage: Box<dyn StorageEngine> = Box::new(MemoryStorage::new());
        let catalog = Box::new(Catalog::new());
        let executor = Box::new(Executor::new(storage, catalog));
        Ok(Self { executor })
    }

    /// 执行一条 SQL
    pub fn execute(&self, sql: &str) -> Result<Vec<Vec<sqlrustgo_common::Value>>> {
        // 1. 词法 + 语法分析
        let mut parser = Parser::new(sql);
        let stmt = parser.parse()?;

        // 2. 逻辑规划
        let planner = Planner::new();
        let logical = planner.plan(stmt)?;

        // 3. 执行
        self.executor.execute(&logical)
    }
}

fn main() {
    println!("SQLRustGo v0.1.0-alpha");
    println!("AI增强的软件工程 - 实验项目 (学号: 202442020106)");
    println!();

    let db = match Database::new_in_memory() {
        Ok(db) => db,
        Err(e) => {
            eprintln!("初始化失败: {e}");
            std::process::exit(1);
        }
    };

    let demo = [
        "CREATE TABLE users (id INT, name TEXT, age INT)",
        "INSERT INTO users VALUES (1, 'Alice', 25)",
        "INSERT INTO users VALUES (2, 'Bob', 17)",
        "INSERT INTO users VALUES (3, 'Carol', 30)",
        "SELECT * FROM users WHERE age > 18",
    ];

    for sql in demo {
        println!("sql> {sql}");
        match db.execute(sql) {
            Ok(rows) => {
                for row in rows {
                    println!("    {row:?}");
                }
            }
            Err(e) => eprintln!("    error: {e}"),
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_database_lifecycle() {
        let db = Database::new_in_memory().unwrap();
        assert!(db.execute("CREATE TABLE t (id INT);").is_ok());
        assert!(db.execute("INSERT INTO t VALUES (1);").is_ok());
        let rows = db.execute("SELECT * FROM t;").unwrap();
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn smoke_select_with_limit_and_offset() {
        let db = Database::new_in_memory().unwrap();
        assert!(db.execute("CREATE TABLE t (id INT);").is_ok());
        for i in 0..10 {
            assert!(db.execute(&format!("INSERT INTO t VALUES ({i});")).is_ok());
        }
        let rows = db.execute("SELECT * FROM t LIMIT 3 OFFSET 2;").unwrap();
        assert_eq!(rows.len(), 3);
    }
}
