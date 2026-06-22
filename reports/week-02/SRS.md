# SQLRustGo 软件需求规格说明书（SRS）

> **版本**：v0.1.0
> **作者**：Tsuki-lyy
> **学号**：202442020106
> **日期**：2026-06-22

---

## 1. 引言

### 1.1 编写目的

本文档规范 SQLRustGo 关系型数据库原型的功能性与非功能性需求，作为后续设计、编码、测试与验收的依据。

### 1.2 项目背景

本项目为「AI 增强的软件工程」课程的实验对象，目标是：
- 让学生用 Rust 从零实现一个支持 DDL/DML/DQL 的极简数据库原型
- 在该原型上实践软件工程全生命周期（需求 → 设计 → 实现 → 测试 → 发布）
- 借助 AI 辅助（TRAE IDE）完成工程化交付

### 1.3 定义与缩写

| 术语 | 解释 |
|------|------|
| DDL | Data Definition Language，建表/删表 |
| DML | Data Manipulation Language，增/改/删 |
| DQL | Data Query Language，查询 |
| AST | Abstract Syntax Tree，抽象语法树 |
| SRS | Software Requirement Specification |

### 1.4 参考资料

- ISO/IEC 25010 — 系统与软件质量模型
- 《软件工程导论》（张海藩）
- Rust Reference 1.93

---

## 2. 项目概述

### 2.1 目标

- 完整支持 `CREATE/DROP/INSERT/SELECT/UPDATE/DELETE` 六类语句
- 内存存储引擎，支持并发安全
- 配套 6 个 crate 的 workspace 结构，覆盖工程化全流程

### 2.2 用户特征

| 角色 | 描述 | 主要诉求 |
|------|------|----------|
| 学生 | 课程学习者 | 易读、可改、可测 |
| 教师 | 课程评分者 | 可批量执行测试、生成质量报告 |
| 维护者 | 后续贡献者 | 模块边界清晰、CI 健全 |

### 2.3 运行环境

| 项 | 要求 |
|----|------|
| OS | Windows 10+/Linux/macOS |
| Rust | 1.93+ |
| 内存 | ≥ 256MB 可用 |
| 磁盘 | ≥ 100MB（构建产物） |

---

## 3. 功能性需求

### FR-1 DDL

**3.1 CREATE TABLE**

| 项 | 描述 |
|----|------|
| 输入 | `CREATE TABLE <name> ( <col> <type>, ... )` |
| 校验 | 表名不重复、列名不重复、类型合法 |
| 输出 | 新表已建；返回 `[]` |
| 异常 | `table '<name>' already exists` |

**3.2 DROP TABLE**

| 项 | 描述 |
|----|------|
| 输入 | `DROP TABLE <name>` |
| 校验 | 表存在 |
| 输出 | 表已删除；返回 `[]` |
| 异常 | `table '<name>' not found` |

### FR-2 DML

**3.3 INSERT**

| 项 | 描述 |
|----|------|
| 输入 | `INSERT INTO <name> VALUES ( v1, v2, ... )` |
| 校验 | 表存在、值数量等于列数量 |
| 输出 | 返回 `[]` |
| 异常 | 列数不匹配、类型错误 |

**3.4 UPDATE**

| 项 | 描述 |
|----|------|
| 输入 | `UPDATE <name> SET <col>=<val> [WHERE <expr>]` |
| 校验 | 表存在、列存在、表达式可求值 |
| 输出 | 返回 `[[<affected_rows>]]` |
| 异常 | 列不存在、类型不匹配 |

**3.5 DELETE**

| 项 | 描述 |
|----|------|
| 输入 | `DELETE FROM <name> [WHERE <expr>]` |
| 输出 | 返回 `[[<affected_rows>]]` |

### FR-3 DQL

**3.6 SELECT**

| 项 | 描述 |
|----|------|
| 输入 | `SELECT * FROM <name> [WHERE <expr>]` |
| 输出 | 满足条件的行集合 |
| 排序 | 不要求（留作 Week 12 性能优化） |

### FR-4 数据类型

| Rust 类型 | SQL 名称 | 取值范围 |
|-----------|----------|----------|
| `i64` | INT | -2^63 ~ 2^63-1 |
| `f64` | FLOAT | IEEE 754 双精度 |
| `String` | TEXT | UTF-8 任意长度 |
| `bool` | BOOL | true / false |
| - | NULL | 缺省值 |

### FR-5 内存存储

- 主结构：`HashMap<TableName, Table>`，其中 `Table = { columns, rows: Vec<Vec<Value>> }`
- 并发：每个表/全局 HashMap 由 `RwLock` 保护

### FR-6 元数据目录

- `Catalog` 记录所有已建表的列名与类型
- 用来支持 `UPDATE/SELECT` 时做语义校验

### FR-7 错误处理

统一 `Error` 枚举（位于 `sqlrustgo-common::error`）：

```
enum Error {
    Lexer(String),
    Parse(String),
    Semantic(String),
    Execution(String),
    Storage(String),
    Unimplemented(String),
}
```

通过 `thiserror` derive `std::error::Error`，向上返回 `Result<T, Error>`。

### FR-8 并发安全

- 内存存储与元数据都用 `std::sync::RwLock`
- 读多写少场景下性能良好

---

## 4. 非功能性需求

| 编号 | 类别 | 指标 | 验证方法 |
|------|------|------|----------|
| NFR-1 | 性能 | 1000 行 SCAN < 50ms | cargo bench |
| NFR-2 | 内存 | 1k 行 < 10MB | heaptrack |
| NFR-3 | 可用性 | CLI 错误信息带建议 | 手动 |
| NFR-4 | 可测试性 | 行覆盖 ≥ 70% | cargo tarpaulin |
| NFR-5 | 可维护性 | clippy 0 warning | CI |
| NFR-6 | 安全性 | 无 unsafe | 静态检查 |
| NFR-7 | 可移植性 | CI 三平台 | GitHub Actions |

---

## 5. 约束

| 编号 | 描述 |
|------|------|
| C-1 | Rust 2021 edition，stable ≥ 1.93 |
| C-2 | 单一可执行文件，0 运行时依赖 |
| C-3 | MIT 协议 |
| C-4 | 不使用网络、文件系统外部接口 |

---

## 6. 验收准则

- ✅ `cargo build --all` 无 error
- ✅ `cargo test --all` 全部通过
- ✅ `cargo clippy --all -- -D warnings` 通过
- ✅ Demo 5 条 SQL 全程无误运行
