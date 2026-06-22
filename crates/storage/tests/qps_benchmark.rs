//! QPS 基准测试（Week 10 Harness 治理）
//!
//! 通过 Database 顶层入口跑 QPS，用于 Gate BP2 检查。
//! 由于集成慢，默认 `#[ignore]`，使用 `--ignored --nocapture` 触发。
use std::time::Instant;

use sqlrustgo_common::Value;
use sqlrustgo_parser::Expr;
use sqlrustgo_storage::{MemoryStorage, StorageEngine};

const BENCH_ITER: usize = 5_000; // 本机 5k 行；远端再放大

fn fresh() -> MemoryStorage {
    let s = MemoryStorage::new();
    s.create_table(
        "users",
        vec![
            ("id".into(), "INT".into()),
            ("name".into(), "TEXT".into()),
            ("age".into(), "INT".into()),
        ],
    )
    .unwrap();
    s
}

fn run_bench<F: FnMut()>(name: &str, mut op: F) -> f64 {
    let start = Instant::now();
    op();
    let dur = start.elapsed();
    let qps = BENCH_ITER as f64 / dur.as_secs_f64();
    println!("{name} QPS: {BENCH_ITER} queries in {dur:?} ({qps:.2} qps)");
    qps
}

#[test]
#[ignore]
fn qps_insert() {
    let s = fresh();
    let q = run_bench("INSERT", || {
        for i in 0..BENCH_ITER {
            let _ = s.insert(
                "users",
                vec![
                    Value::Int(i as i64),
                    Value::Text(format!("n_{i}")),
                    Value::Int(30),
                ],
            );
        }
    });
    assert!(q > 0.0);
}

#[test]
#[ignore]
fn qps_select() {
    let s = fresh();
    for i in 0..BENCH_ITER {
        let _ = s.insert(
            "users",
            vec![
                Value::Int(i as i64),
                Value::Text(format!("n_{i}")),
                Value::Int(30),
            ],
        );
    }
    let q = run_bench("SELECT", || {
        for _ in 0..BENCH_ITER {
            let _ = s.scan("users");
        }
    });
    assert!(q > 0.0);
}

#[test]
#[ignore]
fn qps_update() {
    let s = fresh();
    for i in 0..BENCH_ITER {
        let _ = s.insert(
            "users",
            vec![
                Value::Int(i as i64),
                Value::Text(format!("n_{i}")),
                Value::Int(30),
            ],
        );
    }
    let filter = Expr::Binary {
        left: Box::new(Expr::Column("id".into())),
        op: sqlrustgo_parser::BinOp::Ge,
        right: Box::new(Expr::Literal(Value::Int(0))),
    };
    let q = run_bench("UPDATE", || {
        for i in 0..BENCH_ITER {
            let _ = s.update(
                "users",
                vec![("age".into(), Value::Int((i % 100) as i64))],
                Some(&filter),
            );
        }
    });
    assert!(q > 0.0);
}

#[test]
#[ignore]
fn qps_delete() {
    let s = fresh();
    for i in 0..BENCH_ITER {
        let _ = s.insert(
            "users",
            vec![
                Value::Int(i as i64),
                Value::Text(format!("n_{i}")),
                Value::Int(30),
            ],
        );
    }
    // 注意：每条都会重置表，便于统计
    let q = run_bench("DELETE", || {
        for i in 0..BENCH_ITER {
            let filter = Expr::Binary {
                left: Box::new(Expr::Column("id".into())),
                op: sqlrustgo_parser::BinOp::Eq,
                right: Box::new(Expr::Literal(Value::Int(i as i64))),
            };
            let _ = s.delete("users", Some(&filter));
        }
    });
    assert!(q > 0.0);
}
