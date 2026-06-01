# SQLRustGo 1.0 测试计划

## 1. 测试目标

### 1.1 功能测试目标

确保SQLRustGo数据库系统的核心功能正确实现，包括：

- **SQL解析**：正确解析各种SQL语句（SELECT、INSERT、UPDATE、DELETE等）
- **查询优化**：优化器能正确生成高效的执行计划
- **查询执行**：执行引擎能正确执行物理计划并返回正确结果
- **数据存储**：存储引擎能正确持久化数据，支持事务ACID特性
- **并发控制**：正确处理并发事务，保证数据一致性

### 1.2 性能测试目标

验证系统在不同负载下的性能表现：

- **吞吐量**：每秒处理的查询数量
- **响应时间**：查询的平均响应时间和最大响应时间
- **资源利用率**：CPU、内存、磁盘IO的使用情况
- **扩展性**：系统在增加负载时的性能变化

### 1.3 可靠性测试目标

确保系统在异常情况下的稳定性：

- **容错能力**：系统能处理各种异常情况（网络中断、磁盘错误等）
- **数据完整性**：崩溃后数据不丢失，事务能正确恢复
- **并发正确性**：多事务并发执行时数据保持一致
- **边界条件**：正确处理极端输入和边界情况

## 2. 测试策略

### 2.1 单元测试

**目标**：验证每个模块的独立功能正确性

**覆盖范围**：
- Parser模块：词法分析、语法分析、AST生成
- Optimizer模块：逻辑优化、物理优化、成本估算
- Executor模块：算子执行、结果集处理
- Storage模块：数据读写、事务管理、锁机制

**策略**：
- 每个函数/方法至少有一个测试用例
- 覆盖正常路径和异常路径
- 使用mock对象隔离依赖

### 2.2 集成测试

**目标**：验证模块间协作的正确性

**覆盖范围**：
- Parser → Optimizer：逻辑计划生成和传递
- Optimizer → Executor：物理计划生成和执行
- Executor → Storage：数据读写和事务管理
- Parser → Optimizer → Executor → Storage：完整查询流程

**策略**：
- 测试模块间接口的正确性
- 验证数据在模块间传递的完整性
- 测试错误处理的传播机制

### 2.3 端到端测试

**目标**：验证完整SQL查询流程的正确性

**覆盖范围**：
- 完整的SELECT查询流程
- 完整的INSERT/UPDATE/DELETE流程
- 事务的ACID特性验证
- 并发事务处理

**策略**：
- 模拟真实用户场景
- 使用真实数据进行测试
- 验证端到端的正确性和性能

## 3. 测试用例设计

### 3.1 Parser模块测试用例

| 测试用例ID | 测试名称 | 测试描述 | 预期结果 |
|-----------|---------|---------|---------|
| PARSER-001 | 基本SELECT解析 | 解析简单SELECT语句 `SELECT * FROM users` | 生成正确的AST，包含TableScan算子 |
| PARSER-002 | 带条件的SELECT解析 | 解析带WHERE条件的SELECT `SELECT name FROM users WHERE age > 18` | 生成正确的AST，包含Filter算子 |
| PARSER-003 | JOIN语句解析 | 解析INNER JOIN语句 `SELECT * FROM users JOIN orders ON users.id = orders.user_id` | 生成正确的AST，包含Join算子 |
| PARSER-004 | 聚合函数解析 | 解析带GROUP BY的SELECT `SELECT COUNT(*), department FROM employees GROUP BY department` | 生成正确的AST，包含Aggregate算子 |
| PARSER-005 | 语法错误处理 | 解析语法错误的SQL `SELECT FROM users` | 返回明确的语法错误信息，包含错误位置 |
| PARSER-006 | SQL方言支持 | 解析MySQL特有语法 `SELECT * FROM users LIMIT 10 OFFSET 5` | 正确解析并生成AST |
| PARSER-007 | 复杂表达式解析 | 解析包含复杂表达式的WHERE子句 `SELECT * FROM items WHERE price > 100 AND (category = 'A' OR category = 'B')` | 生成正确的AST，表达式结构正确 |

### 3.2 Optimizer模块测试用例

| 测试用例ID | 测试名称 | 测试描述 | 预期结果 |
|-----------|---------|---------|---------|
| OPT-001 | 谓词下推优化 | 验证谓词下推规则正确应用 | Filter算子被下推到TableScan算子下方 |
| OPT-002 | 投影下推优化 | 验证投影下推规则正确应用 | Project算子被下推，只读取需要的列 |
| OPT-003 | 常量折叠优化 | 验证常量折叠规则正确应用 | 常量表达式在编译时被计算 |
| OPT-004 | Join重排序 | 验证Join顺序优化 | 选择成本最低的Join顺序 |
| OPT-005 | 索引选择 | 验证索引扫描选择正确 | 当存在合适索引时选择IndexScan |
| OPT-006 | 成本估算准确性 | 验证成本模型估算的准确性 | 选择的执行计划实际执行时间最短 |
| OPT-007 | 多规则组合优化 | 验证多个优化规则组合应用 | 多个规则按优先级顺序应用，产生最优计划 |

### 3.3 Executor模块测试用例

| 测试用例ID | 测试名称 | 测试描述 | 预期结果 |
|-----------|---------|---------|---------|
| EXEC-001 | 表扫描执行 | 执行简单SELECT查询 | 返回正确的结果集 |
| EXEC-002 | Filter算子执行 | 执行带WHERE条件的SELECT | 只返回满足条件的行 |
| EXEC-003 | Project算子执行 | 执行带投影的SELECT | 只返回指定的列 |
| EXEC-004 | Join算子执行 | 执行INNER JOIN查询 | 返回正确的连接结果 |
| EXEC-005 | Aggregate算子执行 | 执行带GROUP BY的SELECT | 正确计算聚合结果 |
| EXEC-006 | Sort算子执行 | 执行带ORDER BY的SELECT | 结果按指定顺序排列 |
| EXEC-007 | Limit算子执行 | 执行带LIMIT的SELECT | 返回指定数量的行 |
| EXEC-008 | 复杂查询执行 | 执行包含多个算子的复杂查询 | 返回正确的结果 |

### 3.4 Storage模块测试用例

| 测试用例ID | 测试名称 | 测试描述 | 预期结果 |
|-----------|---------|---------|---------|
| STOR-001 | 数据插入 | 插入一条记录到表中 | 数据正确持久化，可查询到 |
| STOR-002 | 数据更新 | 更新已存在的记录 | 更新后数据正确，旧数据不可访问 |
| STOR-003 | 数据删除 | 删除一条记录 | 删除后数据不可查询 |
| STOR-004 | 事务提交 | 执行提交事务 | 事务中修改的数据持久化 |
| STOR-005 | 事务回滚 | 执行回滚事务 | 事务中修改的数据被撤销 |
| STOR-006 | WAL恢复 | 模拟崩溃后恢复 | 数据完整恢复，未提交事务被回滚 |
| STOR-007 | 并发写入 | 多个事务同时写入 | 数据一致性保持，无数据丢失 |
| STOR-008 | 锁机制测试 | 测试读锁和写锁 | 正确的锁等待和释放行为 |

### 3.5 端到端测试用例

| 测试用例ID | 测试名称 | 测试描述 | 预期结果 |
|-----------|---------|---------|---------|
| E2E-001 | 完整SELECT流程 | 从SQL输入到结果输出的完整流程 | 正确解析、优化、执行并返回结果 |
| E2E-002 | 完整INSERT流程 | 插入数据并验证 | 数据正确持久化，可查询 |
| E2E-003 | 完整UPDATE流程 | 更新数据并验证 | 更新后数据正确 |
| E2E-004 | 完整DELETE流程 | 删除数据并验证 | 删除后数据不可查询 |
| E2E-005 | 事务ACID测试 | 测试事务的原子性、一致性、隔离性、持久性 | 所有ACID特性正确实现 |
| E2E-006 | 并发事务测试 | 多个事务并发执行 | 数据一致性保持，无死锁 |
| E2E-007 | 边界条件测试 | 测试极端输入（大数据量、空表等） | 系统正确处理，不崩溃 |

## 4. 测试环境

### 4.1 硬件环境

| 配置项 | 开发环境 | 测试环境 | 生产环境 |
|-------|---------|---------|---------|
| CPU | Intel i5-8GB | Intel i7-16GB | Intel Xeon-32GB+ |
| 内存 | 8GB | 16GB | 32GB+ |
| 磁盘 | SSD 256GB | SSD 512GB | SSD 1TB+ |
| 网络 | 100Mbps | 1Gbps | 10Gbps |

### 4.2 软件环境

| 配置项 | 版本 |
|-------|------|
| 操作系统 | Ubuntu 22.04 LTS / Windows 10 |
| Rust版本 | 1.75.0 |
| Cargo版本 | 1.75.0 |
| 数据库 | SQLite（用于对比测试） |
| 测试框架 | Rust标准测试框架 + criterion（性能测试） |

### 4.3 测试工具

| 工具名称 | 用途 |
|---------|------|
| `cargo test` | 单元测试和集成测试 |
| `cargo bench` | 性能基准测试 |
| `criterion` | 精确的性能测试 |
| `tarpaulin` | 代码覆盖率分析 |
| `mockall` | Mock对象生成 |
| `docker` | 环境隔离和部署 |

## 5. 测试覆盖率目标

### 5.1 代码覆盖率

| 模块 | 目标覆盖率 | 说明 |
|-----|-----------|------|
| Parser模块 | ≥ 85% | 覆盖主要解析路径 |
| Optimizer模块 | ≥ 80% | 覆盖主要优化规则 |
| Executor模块 | ≥ 85% | 覆盖所有算子 |
| Storage模块 | ≥ 80% | 覆盖事务和锁机制 |
| 整体 | ≥ 82% | 项目整体覆盖率 |

### 5.2 测试类型覆盖率

| 测试类型 | 覆盖要求 |
|---------|---------|
| 单元测试 | 每个公共API至少一个测试用例 |
| 集成测试 | 覆盖所有模块间接口 |
| 端到端测试 | 覆盖主要业务场景 |
| 性能测试 | 覆盖核心查询场景 |
| 可靠性测试 | 覆盖异常和边界情况 |

## 6. 自动化测试方案

### 6.1 测试组织

```
tests/
├── unit/
│   ├── parser/          # Parser模块单元测试
│   ├── optimizer/       # Optimizer模块单元测试
│   ├── executor/        # Executor模块单元测试
│   └── storage/         # Storage模块单元测试
├── integration/         # 集成测试
│   ├── parser_optimizer.rs
│   ├── optimizer_executor.rs
│   └── executor_storage.rs
├── e2e/                 # 端到端测试
│   ├── query_tests.rs
│   ├── transaction_tests.rs
│   └── concurrency_tests.rs
└── benchmarks/          # 性能测试
    ├── parser_benchmark.rs
    ├── optimizer_benchmark.rs
    └── executor_benchmark.rs
```

### 6.2 测试运行命令

```bash
# 运行所有单元测试
cargo test --tests

# 运行特定模块的单元测试
cargo test --tests parser::tests

# 运行集成测试
cargo test --tests integration

# 运行端到端测试
cargo test --tests e2e

# 运行性能测试
cargo bench

# 生成覆盖率报告
cargo tarpaulin --out Html
```

### 6.3 CI/CD集成

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Run unit tests
      run: cargo test --tests
    
    - name: Run integration tests
      run: cargo test --tests integration
    
    - name: Run e2e tests
      run: cargo test --tests e2e
    
    - name: Generate coverage report
      run: cargo tarpaulin --out Html
    
    - name: Upload coverage report
      uses: actions/upload-artifact@v4
      with:
        name: coverage-report
        path: tarpaulin-report.html
```

### 6.4 测试数据管理

```rust
// tests/test_data.rs
pub mod test_data {
    use crate::storage::Record;
    
    pub fn get_test_users() -> Vec<Record> {
        vec![
            Record::new(vec![
                Value::Int(1),
                Value::String("Alice".to_string()),
                Value::Int(25),
            ]),
            Record::new(vec![
                Value::Int(2),
                Value::String("Bob".to_string()),
                Value::Int(30),
            ]),
            Record::new(vec![
                Value::Int(3),
                Value::String("Charlie".to_string()),
                Value::Int(35),
            ]),
        ]
    }
    
    pub fn get_test_schema() -> Schema {
        Schema::new(vec![
            Column::new("id".to_string(), DataType::Int, false),
            Column::new("name".to_string(), DataType::String, false),
            Column::new("age".to_string(), DataType::Int, true),
        ])
    }
}
```

## 7. 测试进度计划

### 7.1 测试阶段划分

| 阶段 | 时间 | 目标 |
|-----|------|------|
| 单元测试 | 第1-2周 | 完成所有模块的单元测试 |
| 集成测试 | 第3周 | 完成模块间集成测试 |
| 端到端测试 | 第4周 | 完成端到端测试 |
| 性能测试 | 第5周 | 完成性能基准测试 |
| 可靠性测试 | 第6周 | 完成可靠性和边界测试 |
| 回归测试 | 持续 | 每次代码提交后自动运行 |

### 7.2 测试里程碑

| 里程碑 | 完成条件 |
|-------|---------|
| M1 | 单元测试覆盖率达到80% |
| M2 | 集成测试全部通过 |
| M3 | 端到端测试全部通过 |
| M4 | 性能测试达到预期指标 |
| M5 | 代码覆盖率达到82% |
| M6 | 所有测试通过，发布候选版本 |

## 8. 测试结果分析

### 8.1 测试报告模板

```markdown
# SQLRustGo测试报告

## 测试概览

| 测试类型 | 测试数量 | 通过 | 失败 | 跳过 |
|---------|---------|------|------|------|
| 单元测试 | 150 | 148 | 2 | 0 |
| 集成测试 | 30 | 29 | 1 | 0 |
| 端到端测试 | 20 | 20 | 0 | 0 |
| 性能测试 | 10 | 10 | 0 | 0 |

## 失败测试详情

| 测试名称 | 失败原因 | 修复建议 |
|---------|---------|---------|
| parser::test_complex_query | 解析嵌套子查询时崩溃 | 修复子查询解析逻辑 |
| integration::test_transaction_isolation | 并发事务数据不一致 | 检查锁机制实现 |

## 性能测试结果

| 测试名称 | 平均响应时间 | P95响应时间 | 吞吐量 |
|---------|-------------|------------|---------|
| select_simple | 1.2ms | 2.5ms | 800 QPS |
| select_complex | 5.8ms | 12.3ms | 170 QPS |
| insert_batch | 0.8ms | 1.5ms | 1200 QPS |

## 代码覆盖率

| 模块 | 覆盖率 |
|-----|-------|
| Parser | 88% |
| Optimizer | 82% |
| Executor | 86% |
| Storage | 81% |
| 整体 | 84% |
```

### 8.2 测试问题跟踪

使用GitHub Issues或Jira跟踪测试失败和缺陷：

- **标签分类**：`bug`, `test-failure`, `performance`, `reliability`
- **优先级**：`P0`（紧急）, `P1`（高）, `P2`（中）, `P3`（低）
- **状态**：`Open`, `In Progress`, `Fixed`, `Closed`

---

**文档版本**: 1.0  
**最后更新**: 2026-06-01  
**维护者**: SQLRustGo开发团队