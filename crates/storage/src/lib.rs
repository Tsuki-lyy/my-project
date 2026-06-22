//! 存储引擎抽象与内存实现
pub mod engine;
pub mod memory;

pub use engine::StorageEngine;
pub use memory::MemoryStorage;
