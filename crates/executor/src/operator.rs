//! 算子接口（迭代器模型：open/next/close）
pub trait Operator {
    fn open(&mut self) -> sqlrustgo_common::Result<()>;
    fn next(&mut self) -> sqlrustgo_common::Result<Option<Vec<sqlrustgo_common::Value>>>;
    fn close(&mut self) -> sqlrustgo_common::Result<()>;
}
