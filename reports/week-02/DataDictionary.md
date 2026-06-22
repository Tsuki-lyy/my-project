# SQLRustGo 数据字典（Data Dictionary）

---

## 1. 数据流

| 编号 | 名称 | 来源 | 去向 | 结构 |
|------|------|------|------|------|
| F1 | SQL 文本 | 用户 | 1.0 词法分析 | `String` (UTF-8) |
| F2 | Token 流 | 1.0 词法分析 | 2.0 语法分析 | `Vec<Token>` |
| F3 | AST | 2.0 语法分析 | 3.0 查询规划 | `enum Statement` |
| F4 | 物理计划 | 3.0 查询规划 | 4.0 执行 | `enum PhysicalPlan` |
| F5 | 结果集 | 4.0 执行 | 用户 | `Vec<Vec<Value>>` |
| F6 | 错误 | 任意处理过程 | 用户 | `enum Error` |
| F7 | 元数据请求 | 4.0 执行 | 5.0 语义检查 | `&str` 表名 |
| F8 | 校验结果 | 5.0 语义检查 | 3.0 查询规划 | `bool` / 列信息 |

---

## 2. 数据存储

### D1 内存表

| 字段 | 类型 | 描述 |
|------|------|------|
| name | `String` | 表名（PK） |
| columns | `Vec<(String, String)>` | 列定义（name, type） |
| rows | `Vec<Vec<Value>>` | 数据行 |

主结构：
```rust
RwLock<HashMap<String, Table>>
```

### D2 元数据目录

| 字段 | 类型 | 描述 |
|------|------|------|
| name | `String` | 表名（PK） |
| columns | `Vec<(String, String)>` | 列信息 |

主结构：
```rust
RwLock<HashMap<String, Vec<(String, String)>>>
```

---

## 3. 数据元素

### 3.1 Token

| 字段 | 类型 | 取值 |
|------|------|------|
| kind | `TokenKind` | Keyword(Keyword) / Ident / Int(i64) / Text(String) / Symbol(char) / Eof |
| lexeme | `String` | 词素原文 |

### 3.2 Value

| 变体 | 内部类型 | 字面量示例 |
|------|----------|------------|
| Null | - | NULL |
| Bool | `bool` | TRUE / FALSE |
| Int | `i64` | 42, -17 |
| Float | `f64` | 3.14 |
| Text | `String` | 'hello' |

### 3.3 Statement（AST 根）

| 变体 | 字段 | 示例 |
|------|------|------|
| CreateTable | name, columns: Vec<ColumnDef> | `CREATE TABLE t (id INT)` |
| DropTable | name | `DROP TABLE t` |
| Insert | table, values: Vec<Value> | `INSERT INTO t VALUES (1, 'a')` |
| Select | table, where_clause: Option<Expr> | `SELECT * FROM t WHERE id = 1` |
| Update | table, assignments, where_clause | `UPDATE t SET x = 1 WHERE ...` |
| Delete | table, where_clause | `DELETE FROM t WHERE id = 1` |

### 3.4 Expr

| 变体 | 字段 | 示例 |
|------|------|------|
| Literal | Value | `42` |
| Column | String | `name` |
| Binary | left, op: BinOp, right | `id = 1` |

### 3.5 BinOp

`Eq / Ne / Lt / Le / Gt / Ge / And / Or`

---

## 4. 别名 / 同义词

| 标准名 | 别名 | 说明 |
|--------|------|------|
| TABLE | - | 唯一 |
| INT | INTEGER | 未来扩展 |
| TEXT | VARCHAR | 未来扩展 |

---

## 5. 数据使用频率与保留期

| 数据 | 频率 | 保留 |
|------|------|------|
| D1 表 | 高（每次查询/写） | 进程生命周期 |
| D2 元数据 | 高（每次语义校验） | 进程生命周期 |

---

## 6. 数据完整性约束

| 约束 | 描述 | 实施位置 |
|------|------|----------|
| 主键唯一 | 表名唯一 | MemoryStorage::create_table |
| NOT NULL | 列值非空 | Value 枚举 Null 变体 |
| 类型一致 | Value 类型与列类型一致 | 解析/执行时校验（Week 06 增强） |
| 引用完整 | 表存在才能操作 | 解析/执行时校验 |
