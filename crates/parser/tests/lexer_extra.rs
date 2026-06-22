//! Lexer 单元测试（Week 08 TDD）
use sqlrustgo_parser::{Keyword, Lexer, TokenKind};

#[test]
fn tokenize_keyword_select_from_where() {
    let toks = Lexer::new("SELECT * FROM users WHERE id = 1")
        .tokenize()
        .expect("lex");
    assert!(matches!(toks[0].kind, TokenKind::Keyword(Keyword::Select)));
    assert!(matches!(toks[1].kind, TokenKind::Symbol('*')));
    assert!(matches!(toks[2].kind, TokenKind::Keyword(Keyword::From)));
    assert!(matches!(toks[3].kind, TokenKind::Ident));
    assert!(matches!(toks[4].kind, TokenKind::Keyword(Keyword::Where)));
}

#[test]
fn tokenize_all_keywords() {
    use sqlrustgo_parser::Keyword::*;
    let sql = "CREATE TABLE DROP INSERT INTO VALUES SELECT FROM WHERE UPDATE SET DELETE INT FLOAT TEXT BOOL AND OR NULL TRUE FALSE";
    let toks = Lexer::new(sql).tokenize().expect("lex");
    let expected = [
        Create, Table, Drop, Insert, Into, Values, Select, From, Where, Update, Set, Delete, Int,
        Float, Text, Bool, And, Or, Null, True, False,
    ];
    for (i, kw) in expected.iter().enumerate() {
        assert!(
            matches!(&toks[i].kind, TokenKind::Keyword(k) if k == kw),
            "token[{}] expected {:?}, got {:?}",
            i,
            kw,
            toks[i].kind
        );
    }
}

#[test]
fn tokenize_string_with_escaped_quote_fails_or_truncates() {
    // 当前 lexer 不支持转义单引号 — 截断到文件末尾
    let toks = Lexer::new("'unterminated").tokenize();
    // 期望错误：未闭合字符串
    assert!(toks.is_err());
}

#[test]
fn tokenize_negative_number_breaks_into_symbol_and_int() {
    // 简化 lexer：'-' 是符号，不在 INT 中
    let toks = Lexer::new("-42").tokenize().expect("lex");
    assert!(matches!(toks[0].kind, TokenKind::Symbol('-')));
    assert!(matches!(toks[1].kind, TokenKind::Int(42)));
}

#[test]
fn tokenize_identifier_with_underscore_and_digits() {
    let toks = Lexer::new("user_id_42").tokenize().expect("lex");
    assert!(matches!(toks[0].kind, TokenKind::Ident));
    assert_eq!(toks[0].lexeme, "user_id_42");
}
