# ADR-006: Release Gate Policy

## 状态
已接受（2026-06-22）

## 决策
7 大类门禁（代码/行为/质量/安全/功能/文档/流程），
技术门禁硬（任何失败 → BLOCK），业务门禁软（缺失 → WARN）。

## 关键阈值
- DELETE/UPDATE QPS ≥ 10,000（E-09 硬性地板）
- 0 RUSTSEC high 漏洞
- 0 unsafe 代码
- 0 panic! / todo!
- 测试覆盖率 ≥ 70%（soft）

## 退出码
- 0: 全部通过 → 可发布
- 1: 至少一个门禁失败 → 阻断合并

## 实施
- 脚本：`scripts/release_gates.ps1`
- 清单：`docs/releases/<version>/RELEASE_GATE_CHECKLIST.md`
- CI：`.gitea/workflows/ci.yml` 在 PR 上自动跑

## 后果
- v3.0.0-alpha 走完 7 大门禁后获 🟡 CONDITIONAL GO
- 推迟 GA 到 v3.0.1，先完成 ADR-007 二级索引
