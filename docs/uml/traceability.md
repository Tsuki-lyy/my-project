# UML ↔ Code 双向追溯矩阵

| UML 元素 | 类型 | 代码位置 |
|----------|------|----------|
| `Database::new_in_memory` | 类 | [src/main.rs](../../src/main.rs) `Database::new_in_memory` |
| `Database::execute` | 类方法 | [src/main.rs](../../src/main.rs) `Database::execute` |
| `Parser::new` | 类方法 | [crates/parser/src/parser.rs](../../crates/parser/src/parser.rs) `Parser::new` |
| `Parser::parse` | 类方法 | [crates/parser/src/parser.rs](../../crates/parser/src/parser.rs) `Parser::parse` |
| `Lexer::tokenize` | 类方法 | [crates/parser/src/lexer.rs](../../crates/parser/src/lexer.rs) `Lexer::tokenize` |
| `Planner::new` | 类方法 | [crates/planner/src/logical.rs](../../crates/planner/src/logical.rs) `Planner::new` |
| `Planner::plan` | 类方法 | [crates/planner/src/logical.rs](../../crates/planner/src/logical.rs) `Planner::plan` |
| `Executor::new` | 类方法 | [crates/executor/src/executor.rs](../../crates/executor/src/executor.rs) `Executor::new` |
| `Executor::execute` | 类方法 | [crates/executor/src/executor.rs](../../crates/executor/src/executor.rs) `Executor::execute` |
| `StorageEngine` | trait | [crates/storage/src/engine.rs](../../crates/storage/src/engine.rs) `trait StorageEngine` |
| `MemoryStorage::new` | 类方法 | [crates/storage/src/memory.rs](../../crates/storage/src/memory.rs) `MemoryStorage::new` |
| `Catalog::new` | 类方法 | [crates/catalog/src/lib.rs](../../crates/catalog/src/lib.rs) `Catalog::new` |
| `Catalog::create_table` | 类方法 | [crates/catalog/src/lib.rs](../../crates/catalog/src/lib.rs) `Catalog::create_table` |
| `Statement::CreateTable` | 枚举 | [crates/parser/src/ast.rs](../../crates/parser/src/ast.rs) `Statement::CreateTable` |
| `Value::Int` | 枚举 | [crates/common/src/value.rs](../../crates/common/src/value.rs) `Value::Int` |
| `Error::Parse` | 枚举 | [crates/common/src/error.rs](../../crates/common/src/error.rs) `Error::Parse` |
