//! MemoryStorage 集成测试（Week 08 TDD）
use sqlrustgo_common::Value;
use sqlrustgo_parser::Expr;
use sqlrustgo_storage::{MemoryStorage, StorageEngine};

fn fresh() -> MemoryStorage {
    let s = MemoryStorage::new();
    s.create_table(
        "t",
        vec![("a".into(), "INT".into()), ("b".into(), "INT".into())],
    )
    .unwrap();
    s
}

#[test]
fn insert_5_rows_scan_returns_5() {
    let s = fresh();
    for i in 0..5 {
        s.insert("t", vec![Value::Int(i), Value::Int(i * 10)])
            .unwrap();
    }
    let rows = s.scan("t").unwrap();
    assert_eq!(rows.len(), 5);
}

#[test]
fn insert_into_unknown_table_errors() {
    let s = MemoryStorage::new();
    assert!(s.insert("missing", vec![Value::Int(1)]).is_err());
}

#[test]
fn create_duplicate_table_errors() {
    let s = fresh();
    let res = s.create_table("t", vec![("x".into(), "INT".into())]);
    assert!(res.is_err());
}

#[test]
fn drop_existing_table_works() {
    let s = fresh();
    assert!(s.drop_table("t").is_ok());
    assert!(s.scan("t").is_err());
}

#[test]
fn drop_unknown_table_errors() {
    let s = MemoryStorage::new();
    assert!(s.drop_table("ghost").is_err());
}

#[test]
fn update_modifies_matching_rows() {
    let s = fresh();
    s.insert("t", vec![Value::Int(1), Value::Int(10)]).unwrap();
    s.insert("t", vec![Value::Int(2), Value::Int(20)]).unwrap();
    s.insert("t", vec![Value::Int(3), Value::Int(30)]).unwrap();

    let filter = Expr::Binary {
        left: Box::new(Expr::Column("a".into())),
        op: sqlrustgo_parser::BinOp::Eq,
        right: Box::new(Expr::Literal(Value::Int(2))),
    };
    let n = s
        .update("t", vec![("b".into(), Value::Int(999))], Some(&filter))
        .unwrap();
    assert_eq!(n, 1);
    let rows = s.scan("t").unwrap();
    let b_values: Vec<&Value> = rows.iter().map(|r| &r[1]).collect();
    assert!(b_values.contains(&&Value::Int(999)));
    assert!(b_values.contains(&&Value::Int(10)));
    assert!(b_values.contains(&&Value::Int(30)));
}

#[test]
fn delete_removes_matching_rows() {
    let s = fresh();
    for i in 0..10 {
        s.insert("t", vec![Value::Int(i), Value::Int(0)]).unwrap();
    }
    // 删除 a < 5
    let filter = Expr::Binary {
        left: Box::new(Expr::Column("a".into())),
        op: sqlrustgo_parser::BinOp::Lt,
        right: Box::new(Expr::Literal(Value::Int(5))),
    };
    let n = s.delete("t", Some(&filter)).unwrap();
    assert_eq!(n, 5);
    let rows = s.scan("t").unwrap();
    assert_eq!(rows.len(), 5);
}
