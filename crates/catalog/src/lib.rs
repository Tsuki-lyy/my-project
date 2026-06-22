//! 元数据目录
use std::collections::HashMap;
use std::sync::RwLock;

pub struct Catalog {
    tables: RwLock<HashMap<String, Vec<(String, String)>>>,
}

impl Catalog {
    pub fn new() -> Self {
        Self { tables: RwLock::new(HashMap::new()) }
    }

    pub fn create_table(&self, name: &str, columns: Vec<(String, String)>) {
        let mut t = self.tables.write().unwrap();
        t.insert(name.to_string(), columns);
    }

    pub fn drop_table(&self, name: &str) -> bool {
        let mut t = self.tables.write().unwrap();
        t.remove(name).is_some()
    }

    pub fn table(&self, name: &str) -> Option<Vec<(String, String)>> {
        self.tables.read().unwrap().get(name).cloned()
    }

    pub fn tables(&self) -> Vec<String> {
        self.tables.read().unwrap().keys().cloned().collect()
    }
}

impl Default for Catalog {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_lifecycle() {
        let c = Catalog::new();
        c.create_table("t", vec![("id".into(), "INT".into())]);
        assert!(c.table("t").is_some());
        assert!(c.drop_table("t"));
        assert!(c.table("t").is_none());
    }
}
