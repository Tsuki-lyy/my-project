//! sqlrustgo 统一错误类型
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("词法错误: {0}")]
    Lexer(String),
    #[error("语法错误: {0}")]
    Parse(String),
    #[error("语义错误: {0}")]
    Semantic(String),
    #[error("执行错误: {0}")]
    Execution(String),
    #[error("存储错误: {0}")]
    Storage(String),
    #[error("未实现: {0}")]
    Unimplemented(String),
}

pub type Result<T> = std::result::Result<T, Error>;
