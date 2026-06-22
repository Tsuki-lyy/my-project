//! 极简语法分析器
use sqlrustgo_common::{Error, Result, Value};

use crate::ast::{BinOp, ColumnDef, DataType, Expr, Statement};
use crate::lexer::{Keyword, Lexer, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(sql: &str) -> Self {
        let tokens = match Lexer::new(sql).tokenize() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("lex error: {e}");
                vec![Token { kind: TokenKind::Eof, lexeme: String::new() }]
            }
        };
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Statement> {
        self.consume_optional_semicolon();
        let stmt = self.parse_statement()?;
        self.consume_optional_semicolon();
        if !matches!(self.peek_kind(), TokenKind::Eof) {
            return Err(Error::Parse("trailing tokens".into()));
        }
        Ok(stmt)
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.peek_kind() {
            TokenKind::Keyword(Keyword::Create) => self.parse_create_table(),
            TokenKind::Keyword(Keyword::Drop) => self.parse_drop_table(),
            TokenKind::Keyword(Keyword::Insert) => self.parse_insert(),
            TokenKind::Keyword(Keyword::Select) => self.parse_select(),
            TokenKind::Keyword(Keyword::Update) => self.parse_update(),
            TokenKind::Keyword(Keyword::Delete) => self.parse_delete(),
            _ => Err(Error::Parse(format!(
                "unexpected token: {:?}",
                self.peek_kind()
            ))),
        }
    }

    fn parse_create_table(&mut self) -> Result<Statement> {
        self.expect_keyword(Keyword::Create)?;
        self.expect_keyword(Keyword::Table)?;
        let name = self.expect_ident()?;
        self.expect_symbol('(')?;
        let mut columns = Vec::new();
        loop {
            let col_name = self.expect_ident()?;
            let ty = self.parse_type()?;
            columns.push(ColumnDef { name: col_name, data_type: ty });
            if matches!(self.peek_kind(), TokenKind::Symbol(',')) {
                self.pos += 1;
            } else {
                break;
            }
        }
        self.expect_symbol(')')?;
        self.consume_optional_semicolon();
        Ok(Statement::CreateTable { name, columns })
    }

    fn parse_drop_table(&mut self) -> Result<Statement> {
        self.expect_keyword(Keyword::Drop)?;
        self.expect_keyword(Keyword::Table)?;
        let name = self.expect_ident()?;
        self.consume_optional_semicolon();
        Ok(Statement::DropTable { name })
    }

    fn parse_insert(&mut self) -> Result<Statement> {
        self.expect_keyword(Keyword::Insert)?;
        self.expect_keyword(Keyword::Into)?;
        let table = self.expect_ident()?;
        self.expect_keyword(Keyword::Values)?;
        self.expect_symbol('(')?;
        let mut values = Vec::new();
        loop {
            values.push(self.parse_value()?);
            if matches!(self.peek_kind(), TokenKind::Symbol(',')) {
                self.pos += 1;
            } else {
                break;
            }
        }
        self.expect_symbol(')')?;
        self.consume_optional_semicolon();
        Ok(Statement::Insert { table, values })
    }

    fn parse_select(&mut self) -> Result<Statement> {
        self.expect_keyword(Keyword::Select)?;
        self.expect_symbol('*')?;
        self.expect_keyword(Keyword::From)?;
        let table = self.expect_ident()?;
        let where_clause = if matches!(self.peek_kind(), TokenKind::Keyword(Keyword::Where)) {
            self.pos += 1;
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.consume_optional_semicolon();
        Ok(Statement::Select { table, where_clause })
    }

    fn parse_update(&mut self) -> Result<Statement> {
        self.expect_keyword(Keyword::Update)?;
        let table = self.expect_ident()?;
        self.expect_keyword(Keyword::Set)?;
        let mut assignments = Vec::new();
        loop {
            let col = self.expect_ident()?;
            self.expect_symbol('=')?;
            let val = self.parse_value()?;
            assignments.push((col, val));
            if matches!(self.peek_kind(), TokenKind::Symbol(',')) {
                self.pos += 1;
            } else {
                break;
            }
        }
        let where_clause = if matches!(self.peek_kind(), TokenKind::Keyword(Keyword::Where)) {
            self.pos += 1;
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.consume_optional_semicolon();
        Ok(Statement::Update { table, assignments, where_clause })
    }

    fn parse_delete(&mut self) -> Result<Statement> {
        self.expect_keyword(Keyword::Delete)?;
        self.expect_keyword(Keyword::From)?;
        let table = self.expect_ident()?;
        let where_clause = if matches!(self.peek_kind(), TokenKind::Keyword(Keyword::Where)) {
            self.pos += 1;
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.consume_optional_semicolon();
        Ok(Statement::Delete { table, where_clause })
    }

    fn parse_type(&mut self) -> Result<DataType> {
        match self.peek_kind() {
            TokenKind::Keyword(Keyword::Int) => { self.pos += 1; Ok(DataType::Int) }
            TokenKind::Keyword(Keyword::Float) => { self.pos += 1; Ok(DataType::Float) }
            TokenKind::Keyword(Keyword::Text) => { self.pos += 1; Ok(DataType::Text) }
            TokenKind::Keyword(Keyword::Bool) => { self.pos += 1; Ok(DataType::Bool) }
            _ => Err(Error::Parse("expected type".into())),
        }
    }

    fn parse_value(&mut self) -> Result<Value> {
        match self.peek_kind() {
            TokenKind::Int(n) => { self.pos += 1; Ok(Value::Int(n)) }
            TokenKind::Text(s) => { self.pos += 1; Ok(Value::Text(s)) }
            TokenKind::Keyword(Keyword::True) => { self.pos += 1; Ok(Value::Bool(true)) }
            TokenKind::Keyword(Keyword::False) => { self.pos += 1; Ok(Value::Bool(false)) }
            TokenKind::Keyword(Keyword::Null) => { self.pos += 1; Ok(Value::Null) }
            _ => Err(Error::Parse("expected literal".into())),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        let left = self.parse_primary_expr()?;
        Ok(left)
    }

    fn parse_primary_expr(&mut self) -> Result<Expr> {
        match self.peek_kind() {
            TokenKind::Int(n) => { self.pos += 1; Ok(Expr::Literal(Value::Int(n))) }
            TokenKind::Text(s) => { self.pos += 1; Ok(Expr::Literal(Value::Text(s))) }
            TokenKind::Ident => {
                let name = match &self.tokens[self.pos].kind {
                    TokenKind::Ident => self.tokens[self.pos].lexeme.clone(),
                    _ => unreachable!(),
                };
                self.pos += 1;
                if matches!(self.peek_kind(), TokenKind::Symbol('=')
                    | TokenKind::Symbol('<') | TokenKind::Symbol('>'))
                {
                    let op = match self.peek_kind() {
                        TokenKind::Symbol('=') => BinOp::Eq,
                        TokenKind::Symbol('<') => BinOp::Lt,
                        TokenKind::Symbol('>') => BinOp::Gt,
                        _ => unreachable!(),
                    };
                    self.pos += 1;
                    let right = self.parse_primary_expr()?;
                    Ok(Expr::Binary {
                        left: Box::new(Expr::Column(name)),
                        op,
                        right: Box::new(right),
                    })
                } else {
                    Ok(Expr::Column(name))
                }
            }
            _ => Err(Error::Parse("expected expression".into())),
        }
    }

    fn peek_kind(&self) -> TokenKind { self.tokens[self.pos].kind.clone() }

    fn expect_keyword(&mut self, kw: Keyword) -> Result<()> {
        if matches!(self.peek_kind(), TokenKind::Keyword(ref k) if k == &kw) {
            self.pos += 1;
            Ok(())
        } else {
            Err(Error::Parse(format!("expected keyword {:?}", kw)))
        }
    }

    fn expect_ident(&mut self) -> Result<String> {
        match self.peek_kind() {
            TokenKind::Ident => {
                let s = self.tokens[self.pos].lexeme.clone();
                self.pos += 1;
                Ok(s)
            }
            other => Err(Error::Parse(format!("expected ident, got {other:?}"))),
        }
    }

    fn expect_symbol(&mut self, c: char) -> Result<()> {
        if matches!(self.peek_kind(), TokenKind::Symbol(s) if s == c) {
            self.pos += 1;
            Ok(())
        } else {
            Err(Error::Parse(format!("expected symbol `{}`", c)))
        }
    }

    fn consume_optional_semicolon(&mut self) {
        while matches!(self.peek_kind(), TokenKind::Symbol(';')) {
            self.pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_create_table() {
        let mut p = Parser::new("CREATE TABLE users (id INT, name TEXT);");
        let s = p.parse().unwrap();
        match s {
            Statement::CreateTable { name, columns } => {
                assert_eq!(name, "users");
                assert_eq!(columns.len(), 2);
            }
            _ => panic!("wrong stmt"),
        }
    }

    #[test]
    fn parse_insert_and_select() {
        let mut p = Parser::new("INSERT INTO users VALUES (1, 'a');");
        assert!(matches!(p.parse().unwrap(), Statement::Insert { .. }));

        let mut p = Parser::new("SELECT * FROM users WHERE id = 1;");
        let s = p.parse().unwrap();
        match s {
            Statement::Select { table, where_clause } => {
                assert_eq!(table, "users");
                assert!(where_clause.is_some());
            }
            _ => panic!("wrong stmt"),
        }
    }
}
