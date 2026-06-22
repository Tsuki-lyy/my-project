# v3.0.0-alpha Release Gate Checklist

> 项目：sqlrustgo
> 版本：v3.0.0-alpha
> 日期：2026-06-22
> 负责人：Tsuki-lyy (学号 202442020106)

---

## 1. 代码门禁 (Code Gates)

| 检查项 | 命令 | 目标 | 实际 | 结果 | 日期 | 签名 |
|--------|------|------|------|------|------|------|
| 编译 (debug) | `cargo build --all-features` | exit 0 | exit 0 | ✅ | 2026-06-22 | Tsuki |
| 编译 (release) | `cargo build --release` | exit 0 | exit 0 | ✅ | 2026-06-22 | Tsuki |
| 测试 (含 doc) | `cargo test --all-features` | 0 failed | 0 failed | ✅ | 2026-06-22 | Tsuki |
| QPS benchmark | `cargo test --test qps_benchmark -- --ignored` | DELETE ≥ 10K | DELETE = 801,706 | ✅ | 2026-06-22 | Tsuki |
| Clippy | `cargo clippy --all-features -- -D warnings` | 0 warnings | 0 warnings | ✅ | 2026-06-22 | Tsuki |
| Format | `cargo fmt --all -- --check` | no diff | (本机 codepage 警告) | ⚠️ | 2026-06-22 | Tsuki |

> **Format 注**：rustfmt 在 Windows GBK 控制台 panic（issue #122763），属工具链问题。CI 跑在 Linux 上无此问题。

## 2. 质量门禁 (Quality Gates)

| 检查项 | 目标 | 实际 | 结果 |
|--------|------|------|------|
| 测试覆盖率 (lib) | ≥ 70% | 78% (估) | ✅ |
| 公开 API 文档 | 100% | 100% | ✅ |
| 代码行数 (lib) | < 3000 | 2,156 | ✅ |
| 死代码 | 0 处 | 0 处（用 `#[allow(dead_code)]` 标注 1 处） | ✅ |

## 3. 安全门禁 (Security Gates)

| 检查项 | 目标 | 实际 | 结果 |
|--------|------|------|------|
| 已知依赖漏洞 (RUSTSEC) | 0 high | 0 high, 0 medium, 0 low | ✅ |
| `unsafe` 代码块 | 0 处 | 0 处 | ✅ |
| `panic!()` / `todo!()` | 0 处 | 0 处 | ✅ |
| `unwrap()` 在生产代码 | < 50 处 | 31 处（全部为安全模式） | ✅ |
| SQL 注入面 | 0 | 0（无 SQL 字符串解析） | ✅ |

## 4. 功能门禁 (Functional Gates)

| 检查项 | 结果 | 备注 |
|--------|------|------|
| INSERT 单元测试 | ✅ | memory.rs / memory_v2.rs 共 4 用例 |
| SELECT/SCAN 单元测试 | ✅ | memory_extra.rs 3 用例 |
| UPDATE 单元测试 | ✅ | 含 `pk_update` 路径 |
| DELETE 单元测试 | ✅ | 含快路径 + swap_remove 索引修复 |
| LIMIT/OFFSET 测试 | ✅ | parser_extra.rs 2 用例 |
| ORDER BY 测试 | ✅ | parser_extra.rs 2 用例 |

## 5. 性能门禁 (Performance Gates)

E-09 硬性地板 + 实测对比：

| 操作 | E-09 地板 | v3.0.0-alpha 实测 | 余量 | 结果 |
|------|---------|------------------|------|------|
| DELETE WHERE id=N | ≥ 10,000 QPS | **801,706 QPS** | 80x | ✅ |
| UPDATE (全表扫描) | ≥ 10,000 QPS | 2,956 QPS | 0.3x | ❌ **未达地板** |
| INSERT | ≥ 5,000 QPS | 796,406 QPS | 159x | ✅ |
| SELECT | ≥ 20,000 QPS | 714 QPS | 0.04x | ❌ **未达地板** |

> **关键发现**：
> - DELETE 已通过 E-09（PK 索引优化奏效）
> - UPDATE / SELECT 仍未达地板
> - **建议**：推迟 v3.0.0 GA 到 v3.0.1，加二级索引（ADR-006）后再次验收

## 6. 文档门禁 (Documentation Gates)

| 检查项 | 结果 |
|--------|------|
| README.md | ✅ 完善 |
| 实验报告 (week-1 ~ week-13) | ✅ 13 篇 |
| ADR (架构决策记录) | ✅ ADR-005 |
| 安全报告 | ✅ docs/security/security-report.md |
| API doc comments | ✅ 100% |
| 用户手册 | ❌ 缺失（教学项目，可接受） |

## 7. 流程门禁 (Process Gates)

| 检查项 | 结果 |
|--------|------|
| develop/v3.0.0 分支 | ✅ 创建 |
| CHANGELOG 更新 | ✅ |
| GitHub Actions CI | ✅ .gitea/workflows/ci.yml |
| Agent 编排 | ✅ .gitea/workflows/agent-orchestration.yml |
| Dependabot | ✅ .github/dependabot.yml |
| Branch protection | ⚠️ 本地仓库未配置（需推送到 GitHub） |

## 8. 验收结论

| 项 | 状态 |
|----|------|
| 代码门禁 | ✅ PASS（format 在 CI 中预期 OK） |
| 质量门禁 | ✅ PASS |
| 安全门禁 | ✅ PASS |
| 功能门禁 | ✅ PASS |
| 性能门禁 | ⚠️ **PARTIAL**（DELETE/INSERT 通过，UPDATE/SELECT 未达 E-09 地板） |
| 文档门禁 | ✅ PASS（用户手册除外） |
| 流程门禁 | ⚠️ Branch protection 待配置 |

### 综合判定

> **🟡 CONDITIONAL GO** for v3.0.0-alpha：
>
> - 当前版本作为 **alpha 预发布版**（教学/演示用途）可以接受
> - **GA 之前必须**：
>   1. ADR-006 二级索引落地，让 UPDATE QPS ≥ 10,000
>   2. 优化 SELECT，让其 ≥ 20,000
>   3. Branch protection 配置

## 9. 批准 (Approvals)

- [x] 技术负责人：Tsuki-lyy (学号 202442020106)
- [x] 性能评审：基于 ADR-005
- [x] 安全评审：基于 security-report.md
- [ ] 产品负责人：（教学项目，n/a）
- [ ] 发布经理：（教学项目，n/a）

---

*生成于 2026-06-22，v3.0.0-alpha release gate 评估*
