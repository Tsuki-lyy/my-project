//! MemoryStorageV2 性能基准 —— 对比 v1 与 v2
//!
//! 验证优化效果：v2 的 DELETE/UPDATE 应当比 v1 快一个数量级
use std::time::Instant;

use sqlrustgo_common::Value;
use sqlrustgo_parser::Expr;
use sqlrustgo_storage::{MemoryStorage, MemoryStorageV2, StorageEngine};

const BENCH_ITER: usize = 5_000;

fn make_v1() -> MemoryStorage {
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

fn make_v2() -> MemoryStorageV2 {
    let s = MemoryStorageV2::new();
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
    println!("{name:30} {BENCH_ITER} iters in {dur:>12?}  ({qps:>10.2} qps)");
    qps
}

fn fill_v1(s: &MemoryStorage) {
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
}

fn fill_v2(s: &MemoryStorageV2) {
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
}

#[test]
#[ignore]
fn bench_v1_delete_pk() {
    let s = make_v1();
    fill_v1(&s);
    let q = run_bench("v1 DELETE WHERE id=N", || {
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

#[test]
#[ignore]
fn bench_v2_delete_pk() {
    let s = make_v2();
    fill_v2(&s);
    let q = run_bench("v2 DELETE WHERE id=N", || {
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

#[test]
#[ignore]
fn bench_v1_update() {
    let s = make_v1();
    fill_v1(&s);
    let filter = Expr::Binary {
        left: Box::new(Expr::Column("id".into())),
        op: sqlrustgo_parser::BinOp::Ge,
        right: Box::new(Expr::Literal(Value::Int(0))),
    };
    let q = run_bench("v1 UPDATE age=...", || {
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
fn bench_v2_update() {
    let s = make_v2();
    fill_v2(&s);
    let filter = Expr::Binary {
        left: Box::new(Expr::Column("id".into())),
        op: sqlrustgo_parser::BinOp::Ge,
        right: Box::new(Expr::Literal(Value::Int(0))),
    };
    let q = run_bench("v2 UPDATE age=...", || {
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
