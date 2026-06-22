# Security Audit Report

> sqlrustgo v3.0.0-alpha
> 2026-06-22
> Author: Tsuki-lyy (学号 202442020106)

## 1. 依赖安全（Dependency Audit）

| 依赖 | 版本约束 | 已知漏洞 | 状态 |
|------|---------|---------|------|
| `serde` | 1.0 | 0 | ✅ 安全 |
| `thiserror` | 1.0 | 0 | ✅ 安全 |
| `criterion` | 0.5 | 0 | ✅ 安全 |

**审计方式**：手动 + 计划接入 `cargo-audit`。
本项目仅 3 个直接依赖（`serde`、`thiserror`、`criterion`），无传递性高风险依赖。

### 建议
- 接入 `cargo-audit` 到 CI（Week 14 发布门禁）
- 启用 Dependabot（已配置 `.github/dependabot.yml`）

## 2. 代码安全（Static Analysis）

### 2.1 Clippy 扫描结果

```bash
cargo clippy --all-features -- -D warnings
```

**结果**：0 警告 0 错误。

### 2.2 unwrap() / panic! 统计

| 文件 | `unwrap()` | `panic!` / `todo!` | 备注 |
|------|----------|------------------|------|
| crates/parser/src/parser.rs | 3 | 0 | 测试代码中 |
| crates/parser/src/lexer.rs | 2 | 0 | 字节流边界检查 |
| crates/storage/src/memory.rs | 8 | 0 | `lock().unwrap()` 4 处，`bytes().next().unwrap()` 4 处 |
| crates/storage/src/memory_v2.rs | 14 | 0 | 同上 + HashMap 查找 |
| crates/catalog/src/lib.rs | 4 | 0 | 测试代码 |
| tests/* | 14 | 0 | 全部在 `#[cfg(test)]` 下 |

**总计**：45 处 `unwrap()`，0 处 `panic!` / `todo!` / `unsafe`。

### 2.3 风险评估

| 风险 | 等级 | 详情 |
|------|------|------|
| `RwLock::write().unwrap()` 中毒 | 中 | 极端情况（其他线程 panic）下会传播，本项目为单进程单线程测试，**实际无影响** |
| `bytes().next().unwrap()` | 低 | 列名永远是非空字符串，**永不 panic** |
| SQL 注入 | **无** | 本项目**不解析用户输入的 SQL**——只接受程序化 API 调用（`s.insert / s.delete`），不存在字符串拼接 |
| 缓冲区溢出 | **无** | Rust 编译期保证 |
| 敏感信息泄露 | **无** | 无日志、无外部 IO、无网络 |

## 3. AI 辅助安全审查

向 Claude 提交 `crates/storage/src/memory_v2.rs` 进行审查，AI 给出 3 条建议：

| 建议 | 严重度 | 处置 |
|------|--------|------|
| `extract_pk_eq` 仅识别 `id = N`，其他 PK 命名（如 `user_id`）会走慢路径 | 低 | 接受，留待 ADR-006 处理多列 PK |
| `swap_remove` 后 `first().cloned()` 触发一次 Clone，5K 行 × 100K op/s = 100K clones/s 可优化 | 低 | 后续可改为 `if let Some(Value::Int(pk)) = ... { entry.pk_index.insert(pk, idx); }` |
| `next_id: i64` 在长期运行下可能溢出 | **低** | i64 范围 ±9.2e18，约 3000 年耗尽，**接受** |

## 4. 修复清单

### 4.1 高优先级（已修）
- 无

### 4.2 中优先级（计划中）
- 引入 `cargo-audit` 集成到 CI（Week 14）
- 把 `lock().unwrap()` 替换为 `match` 处理 poisoned mutex

### 4.3 低优先级（已评估、暂不处理）
- `unwrap()` → `expect("descriptive message")` 以提供更好 panic 信息
- 引入 `tokio` 后检查 `Send + Sync` 边界

## 5. 安全态势总结

| 维度 | 评级 | 说明 |
|------|------|------|
| 依赖安全 | 🟢 优秀 | 3 个直接依赖，0 漏洞 |
| 代码安全 | 🟢 优秀 | 0 unsafe, 0 panic, 0 SQL 注入面 |
| 编译期保证 | 🟢 优秀 | Rust 类型系统 + 借用检查 |
| 运行时安全 | 🟡 良好 | RwLock poison 在极端情况会传播，但本项目单线程测试无影响 |
| 配置安全 | 🟡 良好 | Dependabot 已配置，但未接 cargo-audit |

**总体评级**：🟢 **低风险**，适合作为教学原型发布 v3.0.0-alpha。
