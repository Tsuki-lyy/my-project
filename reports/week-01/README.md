# Week 01 — 实验报告

- [实验报告.md](./实验报告.md) — 详细环境搭建记录
- [检查点.md](./检查点.md) — 7 项任务完成度自检

## 提交

- **Git 分支**：`experiment/week-01-202442020106`
- **Commit**：`chore: bootstrap sqlrustgo workspace skeleton`

## 关键产出

| 项 | 路径 |
|----|------|
| Workspace 配置 | `Cargo.toml` |
| 公共类型 | `crates/common/src/{value,error}.rs` |
| 解析器 | `crates/parser/src/{lexer,parser,ast}.rs` |
| 规划器 | `crates/planner/src/{logical,physical}.rs` |
| 执行器 | `crates/executor/src/{executor,operator}.rs` |
| 存储 | `crates/storage/src/{engine,memory}.rs` |
| 元数据 | `crates/catalog/src/lib.rs` |
| 入口 | `src/main.rs` |
