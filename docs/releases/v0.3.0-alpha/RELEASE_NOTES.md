# sqlrustgo v0.3.0-alpha Release Notes

> Release date: 2026-06-22
> Tag: v0.3.0-alpha
> Target: develop/v0.3.0
> Pre-release: ✅

---

## 发布概述

v0.3.0-alpha 是 sqlrustgo 项目从"原型"走向"工程化"的第一个里程碑。
本版本的核心：**PK 索引 + 完整 CI/CD + Agent 编排 + 发布门禁**。

## 新功能

### 性能
- **MemoryStorageV2**：`crates/storage/src/memory_v2.rs`
  - 引入 PK HashMap 索引
  - DELETE WHERE id=N: 3,003 → **801,706 QPS**（**267x 提速**）
  - 超出 E-09 硬性地板 80x
- **QPS Benchmark**：`crates/storage/tests/qps_benchmark.rs`
  - INSERT / SELECT / UPDATE / DELETE 全覆盖
  - v1 vs v2 对比（`bench_v2.rs`）

### SQL
- **ORDER BY** 支持：parser → planner → executor 全链路
- **LIMIT / OFFSET** 支持
- **Boolean 字面量**支持

### 工具链
- **本地门禁脚本**：`scripts/gate_check.ps1` + `scripts/release_gates.ps1`
- **Gitea Actions CI**：`.gitea/workflows/ci.yml`
  - BP1 静态 / BP2 行为 / BP3 风险 / BP4 文档
- **Agent 编排**：`.gitea/workflows/agent-orchestration.yml`
  - explore / librarian / oracle 三 Agent 并行分析
- **Dependabot**：`.github/dependabot.yml`

### 治理
- **ADR-005**：MemoryStorage PK 索引选型
- **ADR-006**：Release Gate Policy
- **安全报告**：`docs/security/security-report.md`
- **CHANGELOG.md**：完整版本历史
- **ROADMAP.md**：v0.3 → v2.0 长期规划

## 性能数据

| 操作 | v0.2.x | v0.3.0-alpha | 加速比 | E-09 地板 |
|------|--------|--------------|--------|----------|
| DELETE WHERE id=N | 3,003 | **801,706** | 267x | ✅ |
| INSERT | 50,000 | 796,406 | 16x | ✅ |
| UPDATE (full scan) | 2,902 | 2,956 | 1.02x | ❌ |
| SELECT (scan) | 700 | 714 | 1.02x | ❌ |

## 质量

- 0 unsafe 代码
- 0 panic! / todo!
- 0 RUSTSEC 已知漏洞
- 78% 测试覆盖率
- Clippy 0 警告

## 已知问题

1. **UPDATE QPS = 2,956 < E-09 地板 10,000**
   - 处置：推迟 v0.3.0 GA，v0.3.1 通过 ADR-007 二级索引解决
2. **SELECT QPS = 714 < 推荐 20,000**
   - 同上
3. **rustfmt Windows GBK panic**（Rust issue #122763）
   - CI Linux 正常

## 升级指南

本版本为 alpha 预发布，**不建议生产环境使用**。

```toml
# Cargo.toml
[dependencies]
sqlrustgo = { git = "https://github.com/Tsuki-lyy/sqlrustgo", tag = "v0.3.0-alpha" }
```

## 贡献者

@Tsuki-lyy (学号 202442020106)

## 链接

- [CHANGELOG](https://github.com/Tsuki-lyy/sqlrustgo/blob/develop/CHANGELOG.md)
- [ROADMAP](https://github.com/Tsuki-lyy/sqlrustgo/blob/develop/ROADMAP.md)
- [安全报告](https://github.com/Tsuki-lyy/sqlrustgo/blob/develop/docs/security/security-report.md)
- [发布门禁](https://github.com/Tsuki-lyy/sqlrustgo/blob/develop/docs/releases/v0.3.0-alpha/RELEASE_GATE_CHECKLIST.md)
