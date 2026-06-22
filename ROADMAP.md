# sqlrustgo Roadmap (2026 - 2027)

> 课程：软件工程导论 · 实验 15
> 维护者：Tsuki-lyy
> 周期：2026-Q2 ~ 2027-Q4

---

## 一、版本路线图

| 版本 | 目标 | 计划时间 | 状态 |
|------|------|---------|------|
| v0.1.0 | 项目骨架 | 2026-Q1 | ✅ 已完成 |
| v0.2.0 | 解析器 + 执行器 v1 | 2026-Q1 | ✅ 已完成 |
| **v0.3.0-alpha** | PK 索引 + CI/CD + Agent 编排 | **2026-Q2** | 🟡 当前 |
| v0.3.1 | 二级索引 + UPDATE/SELECT 达 E-09 | 2026-Q3 | ⬜ 计划 |
| v0.4.0 | JOIN + 聚合函数 | 2026-Q3 | ⬜ 计划 |
| v0.5.0 | 子查询 + 视图 | 2026-Q4 | ⬜ 计划 |
| v0.6.0 | 持久化（B+Tree + WAL） | 2026-Q4 | ⬜ 计划 |
| **v1.0.0 GA** | 完整 SQL-92 子集 + 持久化 + 文档 | **2027-Q1** | ⬜ 目标 |
| v1.1.0 | TCP 监听 + 自定义协议 | 2027-Q2 | ⬜ 远期 |
| v2.0.0 | 分布式 / 云原生（实验性） | 2027-Q4 | ⬜ 探索 |

---

## 二、技术规划

### 2.1 短期（v0.3.0 → v0.3.1，本季度内）

- **ADR-007 二级索引**：BTreeMap<K, usize> on non-PK columns
- **优化目标**：UPDATE QPS ≥ 10,000，SELECT QPS ≥ 20,000
- **cargo-audit 集成**：CI 增加 `cargo audit --deny warnings`
- **覆盖率提升**：从 78% 推到 85%

### 2.2 中期（v0.4.0 ~ v0.6.0，3-6 个月）

- **SQL 完整度**：
  - JOIN（INNER, LEFT, RIGHT）
  - GROUP BY / HAVING / 聚合函数（COUNT/SUM/AVG/MIN/MAX）
  - 子查询（IN / EXISTS / 标量）
  - 视图
- **持久化**：
  - 基于文件的页式存储（4KB / 8KB / 16KB 可配）
  - B+Tree 索引（替代内存 HashMap）
  - WAL（Write-Ahead Log）保证崩溃恢复
- **REPL**：交互式命令行 `sqlrustgo> SELECT * FROM users;`

### 2.3 长期（v1.0.0 ~ v2.0.0，6-12 个月）

- **生产化**：
  - 完整 SQL-92 子集
  - 事务隔离级别（Read Committed + Repeatable Read）
  - 用户/权限系统
  - 性能：> 100K QPS（持久化版本）
- **网络协议**：
  - MySQL wire protocol 兼容
  - PostgreSQL protocol 兼容
  - HTTP/JSON API
- **云原生**（v2.0.0）：
  - Kubernetes operator
  - 分布式查询（基于 Raft）
  - 多租户

---

## 三、生态规划

### 3.1 客户端 SDK
- Rust client（最高优先级）
- Python client（次之）
- Go / TypeScript client（社区驱动）

### 3.2 集成工具
- Grafana 指标导出
- Prometheus exporter
- 备份/恢复 CLI 工具

### 3.3 教学资源
- 完整教科书《构建一个 SQL 数据库》（基于本项目）
- 在线实验环境（Docker + Jupyter）
- 视频教程（12 章节）

---

## 四、社区规划

| 阶段 | 目标 | 关键指标 |
|------|------|---------|
| 2026-Q2 | 内部使用 | 5 个核心贡献者 |
| 2026-Q3 | 公开发布 | GitHub Stars ≥ 100 |
| 2026-Q4 | 早期采用 | 10 个外部项目使用 |
| 2027-Q1 | 生态建立 | 50+ 外部贡献者，3 个第三方 SDK |
| 2027-Q4 | 商业化探索（可选） | 企业 PoC ≥ 3 个 |

---

## 五、风险与对策

| 风险 | 严重度 | 对策 |
|------|--------|------|
| 单人维护 burnout | 高 | 早期招贡献者；ADR 制度化降低 onboarding 成本 |
| 性能瓶颈难突破 | 中 | 引入外部 PR（如借鉴 SQLite/duckdb） |
| 长期未达 v1.0 失去兴趣 | 中 | 阶段性小目标（每 4 周 1 个 release） |
| 教学/生产定位冲突 | 中 | 明确：v0.x 是教学，v1.0+ 是生产 |

---

## 六、OKR (2026 Q2-Q3)

### O1: 完成 v0.3.0 GA
- KR1: ADR-007 二级索引落地，UPDATE QPS ≥ 10K ✅
- KR2: 文档完整度 90%+
- KR3: 1.0 GA 发布

### O2: 建立社区
- KR1: README 英文版
- KR2: CONTRIBUTING.md
- KR3: 第一个外部 PR

### O3: 个人能力成长
- KR1: 完整阅读 1 本 Rust 进阶书
- KR2: 写 1 篇公开技术博客
- KR3: 参与 1 个其他开源项目

---

*生成于 2026-06-22，sqlrustgo v0.3.0-alpha 路线图*
