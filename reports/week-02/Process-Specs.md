# 过程规格说明（PSPEC）

> 为 DFD Level-0 中的每个过程提供算法级描述。

---

## P1 词法分析

| 项 | 内容 |
|----|------|
| 输入 | SQL 字符串 `s` |
| 输出 | `Vec<Token>` |
| 算法 | 状态机：识别关键字 / 标识符 / 数字 / 字符串 / 符号 |
| 错误 | 未识别字符 → `Error::Lexer` |

```
tokenize(s):
    pos = 0
    tokens = []
    while pos < s.len:
        skip whitespace
        c = s[pos]
        if c is alpha or _: read_ident_or_keyword
        elif c is digit: read_number
        elif c == "'": read_string
        elif c is in symbols: emit Symbol(c)
        else: error
    emit EOF
    return tokens
```

---

## P2 语法分析

| 项 | 内容 |
|----|------|
| 输入 | `Vec<Token>` |
| 输出 | `Statement` (AST 根) |
| 算法 | 递归下降 |
| 错误 | 不符合语法 → `Error::Parse` |

```
parse(tokens):
    pos = 0
    skip leading semicolons
    stmt = parse_statement()
    skip trailing semicolons
    assert EOF
    return stmt
```

### 子过程

- `parse_create_table` → 解析列定义循环
- `parse_drop_table`
- `parse_insert` → 解析值列表
- `parse_select` → 可选 WHERE
- `parse_update` → SET 子句 + 可选 WHERE
- `parse_delete` → 可选 WHERE

---

## P3 查询规划

| 项 | 内容 |
|----|------|
| 输入 | `Statement` |
| 输出 | `PhysicalPlan` |
| 算法 | 模式匹配 + 类型填充 |

```
plan(stmt):
    match stmt:
        CreateTable { name, columns } => PhysicalPlan::CreateTable { name, columns: ... }
        DropTable { name } => PhysicalPlan::DropTable { name }
        Insert { table, values } => PhysicalPlan::Insert { table, values }
        Select { table, where_clause } => PhysicalPlan::Project { Scan { table, where_clause } }
        Update { table, assignments, filter } => PhysicalPlan::Update { ... }
        Delete { table, filter } => PhysicalPlan::Delete { ... }
```

---

## P4 执行引擎

| 项 | 内容 |
|----|------|
| 输入 | `PhysicalPlan` |
| 输出 | `Vec<Vec<Value>>` |
| 算法 | 算子调度 |

```
execute(plan):
    match plan:
        CreateTable { name, columns } =>
            catalog.create_table(name, columns)
            storage.create_table(name, columns)
        DropTable { name } =>
            catalog.drop_table(name)
            storage.drop_table(name)
        Insert { table, values } =>
            storage.insert(table, values)
        Project { input } => execute_logical(input)
        Update { ... } => storage.update(...)
        Delete { ... } => storage.delete(...)
```

### 子过程：execute_logical

- `Scan { table, filter }` → 扫表 + 行级过滤
- `Filter { input, predicate }` → 输入 + 行级过滤
- `Project { input }` → 透传

---

## P5 语义检查

| 项 | 内容 |
|----|------|
| 输入 | `Statement` + 元数据 |
| 输出 | 校验通过 / 错误 |
| 算法 | 名字查找 + 类型匹配 |

### 校验项

| 语句 | 校验 |
|------|------|
| CREATE TABLE | 表名不重复、列名不重复、类型合法 |
| DROP TABLE | 表存在 |
| INSERT | 表存在、值数量匹配列数量 |
| SELECT | 表存在、列存在 |
| UPDATE | 表存在、SET 列存在 |
| DELETE | 表存在 |

> v0.1 在执行器中做这些校验；P5 在 Week 06 提取为独立模块。
