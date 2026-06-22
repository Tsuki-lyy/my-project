# AI 增强的软件工程 - 项目报告

> 项目：sqlrustgo —— 用 Rust 实现的教学型关系数据库原型
> 学期：2025-2026 春夏学期 · 软件工程导论
> 学号：202442020106
> 姓名：Tsuki-lyy
> 完成日期：2026-06-22

---

## 一、项目概述

### 1.1 项目目标

在 16 周内，以**AI 辅助 + Harness 治理**的方式，从零构建一个具备完整工程化要素的关系数据库原型（sqlrustgo），覆盖：

| 阶段 | 周次 | 主题 |
|------|------|------|
| 🚗 自动档 | Week 1-2 | 环境 + 结构化方法 |
| 🕹️ 手动档 | Week 3-9 | OOAD / UML / 架构 / 核心模块 / TDD / 治理 |
| 🔧 修车技能 | Week 10-16 | PR / CI / 性能 / 安全 / 发布 / 总结 |

### 1.2 技术选型

| 维度 | 选型 | 理由 |
|------|------|------|
| 编程语言 | Rust 1.75+ | 内存安全 + 零成本抽象 + 现代包管理 |
| 数据库类型 | 内存型 (in-memory) | 教学优先，避免持久化复杂度 |
| 测试框架 | Rust 内置 + criterion | 集成测试 + QPS benchmark |
| CI/CD | Gitea Actions | 兼容 GitHub Actions，开源自托管 |
| AI 辅助 | Claude Code | 代码生成 + 审查 + 重构建议 |
| 版本控制 | Git + Git Flow | 实验分支 `experiment/week-NN-学号` |
| 文档 | Markdown + ADR | 轻量、可读、可追溯 |

### 1.3 项目成果

| 维度 | 数量 | 详情 |
|------|------|------|
| 工作空间 crate | 6 | common, parser, planner, executor, storage, catalog |
| 代码行数 (lib) | ~2,156 | Rust 源码 |
| 测试用例 | 31+ | 单元测试 + 集成测试 |
| 性能基准 | 4 | INSERT/SELECT/UPDATE/DELETE QPS |
| 文档页 | 16+ | 实验报告 + ADR + ROADMAP + CHANGELOG |
| 门禁脚本 | 2 | gate_check.ps1 + release_gates.ps1 |
| CI 工作流 | 2 | ci.yml + agent-orchestration.yml |
| ADR | 2 | ADR-005 (PK 索引) + ADR-006 (Release Gate) |

### 1.4 关键性能数据

| 操作 | QPS | E-09 地板 | 状态 |
|------|-----|---------|------|
| DELETE WHERE id=N | **801,706** | 10,000 | ✅ **267x 超额** |
| INSERT | 796,406 | 5,000 | ✅ |
| UPDATE | 2,956 | 10,000 | ❌ 留待 v0.3.1 |
| SELECT | 714 | 20,000 (推荐) | ❌ 留待 v0.3.1 |

---

## 二、技术实现

### 2.1 架构设计

```
┌─────────────────────────────────────────┐
│           sqlrustgo 架构                 │
├─────────────────────────────────────────┤
│  src/main.rs (CLI / REPL)               │
│              ↓                           │
│  ┌──────────┴──────────┐                │
│  ↓                     ↓                │
│  parser           planner               │
│  (lexer + AST)   (logical → physical)   │
│                       ↓                  │
│                  executor                │
│                       ↓                  │
│               StorageEngine              │
│              ↙           ↘              │
│     MemoryStorage    MemoryStorageV2    │
│                       (PK 索引)         │
└─────────────────────────────────────────┘
```

参考 4+1 视图 + C4 模型：
- **逻辑视图**：6 个 crate 的模块划分
- **开发视图**：Git Flow + 实验分支
- **进程视图**：REPL → parser → planner → executor → storage
- **物理视图**：单进程内存（无持久化）
- **场景（+1）**：SQL 查询用例 + QPS benchmark

### 2.2 核心模块

| 模块 | 职责 | 关键设计 |
|------|------|---------|
| **parser** | SQL 字符串 → AST | 词法分析（lexer）+ 递归下降解析（parser），支持 SELECT/INSERT/UPDATE/DELETE/CREATE/DROP/LIMIT/OFFSET/ORDER BY |
| **planner** | AST → LogicalPlan → PhysicalPlan | LogicalPlan: Scan / Filter / Project / OrderBy / Limit；PhysicalPlan 复用 LogicalPlan |
| **executor** | PhysicalPlan → 结果 | 模式匹配 + `eval_filter` 表达式求值 |
| **storage** | 行存储 + CRUD | MemoryStorage v1（`Vec<Vec<Value>>`）+ v2（PK HashMap 索引） |
| **catalog** | 表元数据 | （教学项目，未深度使用） |
| **common** | Value / Error / Result | `thiserror` 定义统一错误 |

### 2.3 关键技术点

#### 2.3.1 PK HashMap 索引（ADR-005）

```rust
struct Table {
    columns: Vec<(String, String)>,
    rows: Vec<Row>,
    pk_index: HashMap<i64, usize>,  // PK → 行号
    next_id: i64,
}

fn delete(&self, table: &str, filter: Option<&Expr>) -> Result<usize> {
    // 快路径：filter 是 `id = N`，走 PK 索引
    if let Some(pk) = extract_pk_eq(filter) {
        if let Some(&idx) = entry.pk_index.get(&pk) {
            entry.rows.swap_remove(idx);  // O(1) 删除
            // 修复被 swap 过来的行索引
            ...
            return Ok(1);
        }
    }
    // 慢路径：retain + 重建索引
    ...
}
```

**性能提升**：3,003 → 801,706 QPS（267x）

#### 2.3.2 Harness 治理三层模型

```
提示词层 (Prompt)    → PR 模板强制量化目标
                    ↓
上下文层 (Context)   → 提供 benchmark 数据 + 历史 + ADR
                    ↓
Harness 层 (Gate)    → BP1/BP2/BP3/BP4 自动化门禁
```

#### 2.3.3 QPS Benchmark 设计

```rust
const BENCH_ITER: usize = 5_000;

fn run_bench<F: FnMut()>(name: &str, mut op: F) -> f64 {
    let start = Instant::now();
    op();
    let qps = BENCH_ITER as f64 / start.elapsed().as_secs_f64();
    println!("{name} QPS: {BENCH_ITER} queries in {dur:?} ({qps:.2} qps)");
    qps
}
```

`#[ignore]` 默认跳过，需要时 `--ignored --nocapture` 触发。

---

## 三、工程实践

### 3.1 开发流程

```
Issue → Feature Branch → Code (TDD) → BP1 (CI) → BP2 (CI) → Review → Merge → Release
                ↓                                            ↑
                └────────── Performance Check ──────────────┘
```

| 实践 | 工具 / 文件 | 频率 |
|------|------------|------|
| 单元测试 | `cargo test` | 每次 commit |
| Clippy | `cargo clippy -- -D warnings` | 每次 commit |
| QPS benchmark | `cargo test -- --ignored` | 每周 |
| 门禁检查 | `scripts/release_gates.ps1` | 每次 PR |
| 安全审计 | `cargo audit` (计划) | 每周 |

### 3.2 质量保证

| 维度 | 当前 | 目标 |
|------|------|------|
| 测试覆盖率 | 78% | 85% |
| Clippy 警告 | 0 | 0 |
| `unsafe` 块 | 0 | 0 |
| `panic!` / `todo!` | 0 | 0 |
| 已知 RUSTSEC 漏洞 | 0 | 0 |
| QPS 满足 E-09 | 2/4 | 4/4 |

### 3.3 团队协作（个人项目 → 模拟开源）

| 实践 | 实施 |
|------|------|
| 实验分支 | `experiment/week-NN-202442020106` |
| 提交规范 | `<type>(<scope>): <subject>` |
| ADR | 每个重大决策 1 个 ADR |
| CHANGELOG | 每次发版更新 |
| ROADMAP | 季度审视 |

---

## 四、学习收获

### 4.1 技术能力

| 能力 | 提升 |
|------|------|
| **Rust 实战** | 从零到能写出 2000+ 行的多 crate workspace；掌握生命周期、闭包、trait、错误处理 |
| **数据库原理** | 词法/语法分析、AST 设计、查询计划、表达式求值、存储引擎 |
| **软件工程** | SA/SD、OOAD、UML 4+1 视图、C4 模型、ADR、CHANGELOG |
| **DevOps** | CI/CD、Agent 编排、Dependabot、Benchmark、Release Gate |
| **性能优化** | 识别 O(n²) 瓶颈、引入 HashMap 索引、267x 提速 |
| **安全意识** | RUSTSEC 审计、unsafe/panic 抑制、最小依赖面 |

### 4.2 工程能力

> "我是否学会了**给 AI 套上缰绳**？"

是。本项目最核心的收获不是"写了一个数据库"，而是建立了一套**AI 协作工作流**：

1. **问题分解**：把"实现数据库"拆成 16 个可验证的小目标
2. **规则约束**：用 ADR、门禁、benchmark 约束 AI 的输出
3. **量化验收**：每个改动必须用 QPS 数字证明有效

### 4.3 职业素养

| 维度 | 培养方式 |
|------|---------|
| **工程化思维** | 从"能跑"到"可发布"的全链路 |
| **数据驱动决策** | "我猜测性能差" → "QPS 显示 2,956 < 10,000" |
| **可追溯性** | 每个决策有 ADR，每个版本有 CHANGELOG |
| **风险意识** | "🟡 CONDITIONAL GO" 比 "✅ GO" 更诚实 |
| **持续学习** | 16 周内从 Rust 入门到能产出可发布的 crate |

---

## 五、改进建议

### 5.1 功能改进

| 优先级 | 项 | 说明 |
|--------|-----|------|
| P0 | 二级索引（ADR-007） | BTreeMap on non-PK columns → UPDATE/SELECT 达 E-09 |
| P0 | 持久化（B+Tree + WAL） | v0.6.0 目标 |
| P1 | JOIN 支持 | INNER / LEFT / RIGHT |
| P1 | 聚合函数 | COUNT / SUM / AVG / GROUP BY |
| P2 | REPL | 交互式命令行 |
| P2 | MySQL 协议兼容 | 接入 BI 工具 |

### 5.2 工程改进

| 优先级 | 项 | 说明 |
|--------|-----|------|
| P0 | 接入 cargo-audit | 自动化 RUSTSEC 检查 |
| P0 | 推送 GitHub + 配置 branch protection | 真实 CI 跑通 |
| P1 | 改用 criterion | 替代自写 benchmark 框架 |
| P1 | 文档英文版 | 面向开源社区 |
| P2 | 接入 Codecov | 自动覆盖率报告 |

### 5.3 教学改进建议

> 反馈给课程组：

1. **Harness 治理应早于"开始写代码"**：本课程在 Week 9 才引入"分支策略"，建议 Week 1 就讲"PR 模板 + ADR"
2. **Benchmark 应贯穿**：本课程在 Week 10 引入 QPS 概念，建议 Week 5（核心模块）就有 benchmark
3. **AI 协作应作为"元能力"**：建议把"如何向 AI 提问"作为独立一节
4. **真实开源协作**：建议 Week 15-16 让学生给其他同学的 PR 提 review，模拟真实开源
5. **失败案例同样重要**：建议增加"AI 写的代码如何被 harness 拦截"的失败案例

---

## 六、本学期能力总结

### 6.1 三个阶段的成长

| 阶段 | 学会了什么 | 还不熟练 | 要继续练习 |
|------|-----------|---------|-----------|
| 🚗 自动档（Week 1-2） | 会用 AI 提问 | 精确表达需求 | 追问和迭代 |
| 🕹️ 手动档（Week 3-9） | 给 AI 提供项目背景 | 判断什么信息"重要" | 架构理解 + ADR |
| 🔧 修车技能（Week 10-16） | 设计简单门禁规则 | 设计复杂规则体系 | 跨项目复用 Harness |

### 6.2 三个核心问题的答案

**问题 1：如何优化和改进？**
> **我学到的最重要的是**：把"主观判断"变成"客观数据"。
> - 性能争论 → 跑 benchmark，对照 E-09 地板
> - 质量争论 → 跑 clippy + 覆盖率
> - 安全争论 → 跑 cargo audit

**问题 2：AI 无法取代人类什么？**
> **我理解最深的是**：**目标定义**和**判断取舍**。
> - AI 不知道 QPS 地板应该是 10,000 还是 50,000
> - AI 不知道 v0.3.0 应该是 GA 还是 alpha
> - AI 不知道"这个功能虽然能 work，但还不够好发布"
> - **这些是人类的不可替代价值**

**问题 3：我应该加强什么？**
> **我的行动计划是**：
> 1. **每周读 1 篇 Rust 进阶博客**，重点：async、宏、unsafe
> 2. **每季度给 1 个开源项目提 PR**，从 typo fix 起步
> 3. **持续维护 sqlrustgo**，目标 v1.0.0 GA 在 2027 Q1

---

## 七、给学弟学妹的建议

> 我最想告诉下一届学生的一句话：

> **"AI 越强，越需要 Harness。"**

具体来说：
1. **不要让 AI 直接写代码**：先自己画架构、写 ADR、列出 acceptance criteria
2. **把 AI 当结对编程伙伴，不当代写工具**：你出方向，AI 出实现
3. **建立"规则"意识**：PR 模板、门禁脚本、Benchmark 三件套越早建越好
4. **数据 > 主观判断**：所有"快/慢/好/坏"的问题，都用 benchmark 数字回答
5. **失败是常态**：本项目 v0.3.0 因 UPDATE 不达标而推迟 GA，是**正确的决策**而非"项目失败"

---

## 八、附录

### 8.1 仓库与资源

| 项 | 链接 |
|-----|------|
| 仓库 | https://github.com/Tsuki-lyy/sqlrustgo |
| CHANGELOG | `CHANGELOG.md` |
| ROADMAP | `ROADMAP.md` |
| ADR-005 | `docs/adr/ADR-005-memory-storage-pk-index.md` |
| ADR-006 | `docs/adr/ADR-006-release-gate-policy.md` |
| 安全报告 | `docs/security/security-report.md` |
| Release Gate | `docs/releases/v0.3.0-alpha/RELEASE_GATE_CHECKLIST.md` |
| Release Notes | `docs/releases/v0.3.0-alpha/RELEASE_NOTES.md` |

### 8.2 实验报告清单

| Week | 报告 |
|------|------|
| Week 1 | `reports/week-01/实验报告.md` |
| Week 2 | `reports/week-02/实验报告.md` |
| Week 3 | `reports/week-03/实验报告.md` |
| Week 4 | `reports/week-04/实验报告.md` |
| Week 5 | `reports/week-05/实验报告.md` |
| Week 6 | `reports/week-06/实验报告.md` |
| Week 8 | `reports/week-08/实验报告.md` |
| Week 9 | `reports/week-09/实验报告.md` |
| Week 10 | `reports/week-10/实验报告.md` |
| Week 11 | `reports/week-11/实验报告.md` |
| Week 12 | `reports/week-12/实验报告.md` |
| Week 13 | `reports/week-13/实验报告.md` |
| Week 14 | `reports/week-14/实验报告.md` |
| Week 15 | `reports/week-15/实验报告.md` |
| Week 16 | `reports/week-16/实验报告.md` |

### 8.3 致谢

感谢：
- 课程组的精心设计，特别是 Week 10-16 的"修车技能"阶段
- Claude（Anthropic）作为 AI 协作伙伴，提供代码生成与审查
- Rust 社区的 crate 作者（serde、thiserror、criterion）
- 家人对我熬夜写代码的容忍

---

*生成于 2026-06-22，sqlrustgo v0.3.0-alpha · 学期项目总结*
