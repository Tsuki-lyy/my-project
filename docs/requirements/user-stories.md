# SQLRustGo 用户故事

## US-001: 执行SELECT查询
**作为** 数据库用户  
**我想要** 执行SELECT查询语句  
**以便于** 从数据库中检索数据  

**验收标准：**
- 支持 `SELECT * FROM table` 查询所有列
- 支持 `SELECT col1, col2 FROM table` 查询指定列
- 支持 `WHERE` 条件过滤（=, !=, <, >）
- 支持 `ORDER BY` 排序
- 返回正确的结果集

## US-002: 执行INSERT操作
**作为** 数据库用户  
**我想要** 向表中插入新数据  
**以便于** 存储新的记录  

**验收标准：**
- 支持 `INSERT INTO table VALUES (val1, val2)` 语法
- 支持指定列插入 `INSERT INTO table (col1) VALUES (val1)`
- 插入后数据能正确持久化
- 返回插入成功/失败的明确信息

## US-003: 执行UPDATE操作
**作为** 数据库用户  
**我想要** 更新表中已有的数据  
**以便于** 修改现有记录  

**验收标准：**
- 支持 `UPDATE table SET col=val WHERE condition`
- 支持更新多列
- 支持带WHERE条件的有条件更新
- 不支持WHERE时更新所有行

## US-004: 执行DELETE操作
**作为** 数据库用户  
**我想要** 删除表中的数据  
**以便于** 移除不需要的记录  

**验收标准：**
- 支持 `DELETE FROM table WHERE condition`
- 支持带WHERE条件的有条件删除
- 不支持WHERE时删除所有行（清空表）

## US-005: 创建表
**作为** 数据库用户  
**我想要** 创建新表  
**以便于** 定义数据存储结构  

**验收标准：**
- 支持 `CREATE TABLE table (col1 type, col2 type)`
- 支持整数、浮点数、字符串类型
- 表创建后能正常插入和查询