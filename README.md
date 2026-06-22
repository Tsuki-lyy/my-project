# SQLRustGo

> AI增强的软件工程 - 课程实验项目
> 学号: 202442020106
> GitHub: https://github.com/Tsuki-lyy/sqlrustgo

## 项目概述

一个用 Rust 实现的关系型数据库原型，遵循 `Parser → Planner → Executor → Storage` 的分层架构，
并在之上扩展 Catalog/Common 公共组件。本项目作为软件工程导论课程的实验对象，覆盖：

- 需求工程（结构化方法、面向对象分析）
- 架构与设计（UML、模块设计）
- 实现与测试（TDD、性能优化）
- 工程治理（Git 分支、PR、CI/CD、安全扫描、发布门禁）

## 工作区结构

```
sqlrustgo/
├── Cargo.toml              # workspace 配置
├── crates/
│   ├── common/             # 公共类型与错误
│   ├── parser/             # 词法/语法分析
│   ├── planner/            # 逻辑/物理计划
│   ├── executor/           # 执行引擎
│   ├── storage/            # 存储引擎（内存/文件）
│   └── catalog/            # 元数据
├── docs/                   # 设计文档
├── reports/                # 实验报告
├── scripts/                # 门禁/构建脚本
└── .gitea/workflows/       # CI 配置
```

## 快速开始

```bash
cargo build --all-features
cargo test --all-features
cargo clippy --all-features -- -D warnings
cargo fmt -- --check
```

## 实验报告索引

| 周次 | 主题 | 报告 |
|------|------|------|
| 01 | 环境搭建 | [reports/week-01](./reports/week-01/) |
| 02 | 结构化方法 | [reports/week-02](./reports/week-02/) |
| 03 | 面向对象分析 | [reports/week-03](./reports/week-03/) |
| 04 | UML 建模 | [reports/week-04](./reports/week-04/) |
| 05 | 架构设计 | [reports/week-05](./reports/week-05/) |
| 06 | 核心模块 | [reports/week-06](./reports/week-06/) |
| 08 | TDD | [reports/week-08](./reports/week-08/) |
| 09 | 分支治理 | [reports/week-09](./reports/week-09/) |
| 10 | PR/Harness | [reports/week-10](./reports/week-10/) |
| 11 | CI/CD | [reports/week-11](./reports/week-11/) |
| 12 | 性能优化 | [reports/week-12](./reports/week-12/) |
| 13 | 安全审计 | [reports/week-13](./reports/week-13/) |
| 14 | 发布门禁 | [reports/week-14](./reports/week-14/) |
| 15 | 版本发布 | [reports/week-15](./reports/week-15/) |
| 16 | 项目展示 | [reports/week-16](./reports/week-16/) |
