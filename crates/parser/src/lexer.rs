//! 极简词法分析器
use sqlrustgo_common::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Ident,
    Int(i64),
    Float(f64),
    Text(String),
    Symbol(char),
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Create,
    Table,
    Drop,
    Insert,
    Into,
    Values,
    Select,
    From,
    Where,
    Update,
    Set,
    Delete,
    Limit,
    Offset,
    Int,
    Float,
    Text,
    Bool,
    And,
    Or,
    Null,
    True,
    False,
}

pub struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: 0,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>> {
        let mut out = Vec::new();
        while self.pos < self.input.len() {
            let c = self.peek();
            if c.is_ascii_whitespace() {
                self.pos += 1;
                continue;
            }
            if c.is_ascii_alphabetic() || c == b'_' {
                out.push(self.read_ident_or_keyword()?);
                continue;
            }
            if c.is_ascii_digit() {
                out.push(self.read_number()?);
                continue;
            }
            if c == b'\'' {
                out.push(self.read_string()?);
                continue;
            }
            if Self::is_symbol(c) {
                out.push(Token {
                    kind: TokenKind::Symbol(c as char),
                    lexeme: (c as char).to_string(),
                });
                self.pos += 1;
                continue;
            }
            return Err(Error::Lexer(format!("unexpected char `{}`", c as char)));
        }
        out.push(Token {
            kind: TokenKind::Eof,
            lexeme: "".into(),
        });
        Ok(out)
    }

    fn peek(&self) -> u8 {
        self.input[self.pos]
    }

    fn read_ident_or_keyword(&mut self) -> Result<Token> {
        let start = self.pos;
        while self.pos < self.input.len()
            && (self.input[self.pos].is_ascii_alphanumeric() || self.input[self.pos] == b'_')
        {
            self.pos += 1;
        }
        let lexeme = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|_| Error::Lexer("invalid utf-8".into()))?
            .to_string();
        let kind = match lexeme.to_uppercase().as_str() {
            "CREATE" => TokenKind::Keyword(Keyword::Create),
            "TABLE" => TokenKind::Keyword(Keyword::Table),
            "DROP" => TokenKind::Keyword(Keyword::Drop),
            "INSERT" => TokenKind::Keyword(Keyword::Insert),
            "INTO" => TokenKind::Keyword(Keyword::Into),
            "VALUES" => TokenKind::Keyword(Keyword::Values),
            "SELECT" => TokenKind::Keyword(Keyword::Select),
            "FROM" => TokenKind::Keyword(Keyword::From),
            "WHERE" => TokenKind::Keyword(Keyword::Where),
            "UPDATE" => TokenKind::Keyword(Keyword::Update),
            "SET" => TokenKind::Keyword(Keyword::Set),
            "DELETE" => TokenKind::Keyword(Keyword::Delete),
            "LIMIT" => TokenKind::Keyword(Keyword::Limit),
            "OFFSET" => TokenKind::Keyword(Keyword::Offset),
            "INT" => TokenKind::Keyword(Keyword::Int),
            "FLOAT" => TokenKind::Keyword(Keyword::Float),
            "TEXT" => TokenKind::Keyword(Keyword::Text),
            "BOOL" => TokenKind::Keyword(Keyword::Bool),
            "AND" => TokenKind::Keyword(Keyword::And),
            "OR" => TokenKind::Keyword(Keyword::Or),
            "NULL" => TokenKind::Keyword(Keyword::Null),
            "TRUE" => TokenKind::Keyword(Keyword::True),
            "FALSE" => TokenKind::Keyword(Keyword::False),
            _ => TokenKind::Ident,
        };
        Ok(Token { kind, lexeme })
    }

    fn read_number(&mut self) -> Result<Token> {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        // 浮点：遇到 . 继续读
        let mut is_float = false;
        if self.pos < self.input.len()
            && self.input[self.pos] == b'.'
            && self.pos + 1 < self.input.len()
            && self.input[self.pos + 1].is_ascii_digit()
        {
            is_float = true;
            self.pos += 1; // 跳过 .
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }
        let lex = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|_| Error::Lexer("invalid utf-8".into()))?;
        if is_float {
            let v: f64 = lex
                .parse()
                .map_err(|e: std::num::ParseFloatError| Error::Lexer(e.to_string()))?;
            Ok(Token {
                kind: TokenKind::Float(v),
                lexeme: lex.to_string(),
            })
        } else {
            let n: i64 = lex
                .parse()
                .map_err(|e: std::num::ParseIntError| Error::Lexer(e.to_string()))?;
            Ok(Token {
                kind: TokenKind::Int(n),
                lexeme: n.to_string(),
            })
        }
    }

    fn read_string(&mut self) -> Result<Token> {
        self.pos += 1; // 跳过开 '
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] != b'\'' {
            self.pos += 1;
        }
        if self.pos >= self.input.len() {
            return Err(Error::Lexer("unterminated string".into()));
        }
        let s = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|_| Error::Lexer("invalid utf-8".into()))?
            .to_string();
        self.pos += 1; // 跳过闭 '
        Ok(Token {
            kind: TokenKind::Text(s.clone()),
            lexeme: format!("'{s}'"),
        })
    }

    fn is_symbol(c: u8) -> bool {
        matches!(
            c,
            b'(' | b')' | b',' | b'*' | b'=' | b'<' | b'>' | b';' | b'-' | b'+'
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_keywords_and_idents() {
        let toks = Lexer::new("SELECT a FROM t").tokenize().unwrap();
        assert!(matches!(toks[0].kind, TokenKind::Keyword(Keyword::Select)));
        assert!(matches!(toks[1].kind, TokenKind::Ident));
    }

    #[test]
    fn tokenize_numbers_and_strings() {
        let toks = Lexer::new("123 'hi'").tokenize().unwrap();
        assert!(matches!(toks[0].kind, TokenKind::Int(123)));
        assert!(matches!(&toks[1].kind, TokenKind::Text(s) if s == "hi"));
    }
}
