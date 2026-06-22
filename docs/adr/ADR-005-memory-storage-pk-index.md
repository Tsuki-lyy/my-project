# ADR-005: MemoryStorage 主键索引选型

## 状态
已接受（2026-06-22）

## 背景
v1 的 DELETE 操作在 5K 行 5K 次单行删除场景下仅 3,003 QPS，
远低于 E-09 地板 10,000。根因：`Vec::remove` 是 O(n)，
N 次调用累计 O(n²)。

## 决策
引入 `HashMap<i64, usize>` 主键索引，快路径把 DELETE
从 O(n) 降到 O(1)。

## 权衡
- 内存：+16 bytes/行（PK key + index value + HashMap 开销），5K 行约 80KB，可接受
- 复杂度：swap_remove 后必须修复被移动行的索引（一个 if）
- 兼容：原 `retain` 慢路径保留，行为不变

## 后果
- DELETE 提升 267x（3,003 → 801,706 QPS）
- UPDATE 无变化（仍全表扫描，留待 ADR-006）
- 新增测试 `pk_index_repaired_after_swap_remove`

## 验证
```bash
cargo test --package sqlrustgo-storage --test bench_v2 -- --ignored --nocapture
```
输出：
```
v2 DELETE WHERE id=N    5000 iters in 6.2367ms (801706.03 qps)
```
远超 E-09 地板 10,000。
