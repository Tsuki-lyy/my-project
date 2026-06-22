//! Parser 集成测试（Week 08 TDD）
use sqlrustgo_common::Value;
use sqlrustgo_parser::{Parser, Statement};

#[test]
fn parse_create_table_with_three_columns() {
    let mut p = Parser::new("CREATE TABLE t (a INT, b TEXT, c BOOL);");
    let s = p.parse().expect("parse");
    match s {
        Statement::CreateTable { name, columns } => {
            assert_eq!(name, "t");
            assert_eq!(columns.len(), 3);
            assert_eq!(columns[0].name, "a");
            assert_eq!(columns[1].name, "b");
            assert_eq!(columns[2].name, "c");
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn parse_insert_with_mixed_types() {
    let mut p = Parser::new("INSERT INTO t VALUES (1, 'hello', TRUE, 3.14);");
    let s = p.parse().expect("parse");
    match s {
        Statement::Insert { table, values } => {
            assert_eq!(table, "t");
            assert_eq!(values.len(), 4);
            assert_eq!(values[0], Value::Int(1));
            assert_eq!(values[1], Value::Text("hello".to_string()));
            assert_eq!(values[2], Value::Bool(true));
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn parse_select_with_where_and_comparison() {
    let mut p = Parser::new("SELECT * FROM users WHERE age > 18;");
    let s = p.parse().expect("parse");
    match s {
        Statement::Select {
            table,
            where_clause,
        } => {
            assert_eq!(table, "users");
            assert!(where_clause.is_some());
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn parse_update_with_set_and_where() {
    let mut p = Parser::new("UPDATE users SET name = 'Bob' WHERE id = 1;");
    let s = p.parse().expect("parse");
    match s {
        Statement::Update {
            table,
            assignments,
            where_clause,
        } => {
            assert_eq!(table, "users");
            assert_eq!(assignments.len(), 1);
            assert_eq!(assignments[0].0, "name");
            assert!(where_clause.is_some());
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn parse_delete_with_where() {
    let mut p = Parser::new("DELETE FROM users WHERE id = 99;");
    let s = p.parse().expect("parse");
    match s {
        Statement::Delete {
            table,
            where_clause,
        } => {
            assert_eq!(table, "users");
            assert!(where_clause.is_some());
        }
        _ => panic!("expected Delete"),
    }
}

#[test]
fn parse_error_on_unknown_keyword() {
    let mut p = Parser::new("FOOBAR;");
    assert!(p.parse().is_err());
}

#[test]
fn parse_error_on_unclosed_string() {
    let mut p = Parser::new("INSERT INTO t VALUES ('abc);");
    assert!(p.parse().is_err());
}

#[test]
fn parse_error_on_missing_semicolon_via_trailing_tokens() {
    let mut p = Parser::new("INSERT INTO t VALUES (1) extra_garbage;");
    assert!(p.parse().is_err());
}
