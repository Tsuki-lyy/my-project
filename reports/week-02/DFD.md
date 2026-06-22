# 数据流图（DFD）— Context / Level-0 / Level-1

## 1. Context Diagram（顶层）

```mermaid
flowchart LR
    User([User])
    Admin([Admin])
    FS[(File Storage)]

    User -- "SQL 文本 / 元数据指令" --> System((SQLRustGo))
    System -- "结果集 / 错误" --> User
    System -- "执行状态" --> Admin
    System <-. "可选持久化" .-> FS
```

**说明**：

- 顶层图只有 1 个过程（SQLRustGo 系统）
- 2 个外部实体：User、Admin
- 1 个外部存储：File Storage（v0.1 不使用，预留扩展）

---

## 2. Level-0 DFD

```mermaid
flowchart TB
    User([User]) -- "F1: SQL 文本" --> P1[1.0 词法分析]
    P1 -- "F2: Token 流" --> P2[2.0 语法分析]
    P2 -- "F3: AST" --> P3[3.0 查询规划]
    P3 -- "F4: 物理计划" --> P4[4.0 执行引擎]
    P4 -- "F5: 结果集" --> User
    P4 -- "读写" --> D1[(D1: 内存表)]
    P3 -- "F7: 元数据查询" --> P5[5.0 语义检查]
    P5 -- "F8: 校验结果" --> P3
    P5 <-. "F8" .-> D2[(D2: 元数据)]

    P1 -. "F6: 错误" .-> User
    P2 -. "F6" .-> User
    P3 -. "F6" .-> User
    P4 -. "F6" .-> User
```

---

## 3. Level-1 DFD：执行引擎子图

```mermaid
flowchart LR
    Plan([PhysicalPlan]) --> Op1[Op1: CreateTable]
    Plan --> Op2[Op2: DropTable]
    Plan --> Op3[Op3: Insert]
    Plan --> Op4[Op4: Scan + Filter]
    Plan --> Op5[Op5: Update]
    Plan --> Op6[Op6: Delete]
    Plan --> Op7[Op7: Project]

    Op1 --> S[(Storage)]
    Op2 --> S
    Op3 --> S
    Op4 --> S
    Op5 --> S
    Op6 --> S
    Op7 --> S
    S -. "元数据" .-> C[(Catalog)]
```

---

## 4. 数据流条目

| 流编号 | 名称 | 来源 | 去向 | 内容 |
|--------|------|------|------|------|
| F1 | SQL 文本 | User | 1.0 | 字符串 |
| F2 | Token 流 | 1.0 | 2.0 | `Vec<Token>` |
| F3 | AST | 2.0 | 3.0 | `Statement` |
| F4 | 物理计划 | 3.0 | 4.0 | `PhysicalPlan` |
| F5 | 结果集 | 4.0 | User | `Vec<Vec<Value>>` |
| F6 | 错误 | 各过程 | User | `Error` 枚举 |
| F7 | 元数据查询 | 3.0 | 5.0 | 表名/列名 |
| F8 | 校验结果 | 5.0 | 3.0 | `bool` |
