# Changelog

All notable changes to sqlrustgo will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- ADR-007 二级索引（BTreeMap on non-PK columns）
- 更新 QPS 让 UPDATE / SELECT 达到 E-09 地板
- 接入 cargo-audit 到 CI

## [0.3.0-alpha] - 2026-06-22

### Added
- **MemoryStorageV2**：`crates/storage/src/memory_v2.rs` 引入 PK HashMap 索引，DELETE WHERE id=N 提速 **267x**（3,003 → 801,706 QPS）
- **ORDER BY / LIMIT / OFFSET** 完整支持：parser → planner → executor 全链路
- **QPS Benchmark**：`crates/storage/tests/qps_benchmark.rs` 与 `bench_v2.rs`
- **本地 Gate 脚本**：`scripts/gate_check.ps1` 与 `scripts/release_gates.ps1`
- **CI 工作流**：`.gitea/workflows/ci.yml`（BP1-BP4 四级门禁）
- **Agent 编排**：`.gitea/workflows/agent-orchestration.yml`（explore/librarian/oracle）
- **Dependabot**：`.github/dependabot.yml`
- **ADR-005 / ADR-006**：PK 索引选型 + 发布门禁策略
- **安全报告**：`docs/security/security-report.md`
- **发布清单**：`docs/releases/v3.0.0-alpha/RELEASE_GATE_CHECKLIST.md`

### Performance
| Operation | v0.2.x | v0.3.0-alpha | Speedup |
|-----------|--------|--------------|---------|
| DELETE WHERE id=N | 3,003 QPS | **801,706 QPS** | **267x** |
| UPDATE (full scan) | 2,902 QPS | 2,956 QPS | 1.02x |
| INSERT | ~50K QPS | 796K QPS | 16x |
| SELECT (scan) | ~700 QPS | 714 QPS | 1.02x |

### Security
- 0 `unsafe` blocks
- 0 `panic!` / `todo!` in production code
- 0 known RUSTSEC vulnerabilities in 3 direct deps

### Test Coverage
- 31 unit tests in storage
- 8 parser integration tests
- 4 QPS benchmarks (v1 vs v2)
- Estimated 78% line coverage

### Known Issues
- UPDATE QPS = 2,956 < E-09 floor 10,000 (deferred to v0.3.1)
- SELECT QPS = 714 < recommended 20,000 (deferred to v0.3.1)
- rustfmt panics on Windows GBK console (Rust issue #122763); CI on Linux OK

## [0.2.0] - 2026-04-XX

### Added
- Basic parser (lexer + parser) for SELECT/INSERT/UPDATE/DELETE
- Logical planner with Scan/Filter/Project
- Physical executor
- MemoryStorage v1 (`Vec<Vec<Value>>`)

### Known Issues
- DELETE: O(n²) on bulk operations

## [0.1.0] - 2026-03-XX

### Added
- Project skeleton
- Workspace with 6 crates (common, parser, planner, executor, storage, catalog)
- CI scaffolding

[Unreleased]: https://github.com/Tsuki-lyy/sqlrustgo/compare/v0.3.0-alpha...HEAD
[0.3.0-alpha]: https://github.com/Tsuki-lyy/sqlrustgo/releases/tag/v0.3.0-alpha
[0.2.0]: https://github.com/Tsuki-lyy/sqlrustgo/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Tsuki-lyy/sqlrustgo/releases/tag/v0.1.0
