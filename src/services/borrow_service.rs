use std::collections::HashMap;

use crate::models::BorrowRecord;

pub struct BorrowService {
    records: HashMap<String, BorrowRecord>,
    next_id: u32,
}

impl BorrowService {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn borrow_book(&mut self, user_id: &str, book_isbn: &str) -> String {
        let record_id = format!("BR{}", self.next_id);
        self.next_id += 1;

        let record = BorrowRecord::new(record_id.clone(), user_id.to_string(), book_isbn.to_string());
        self.records.insert(record_id.clone(), record);
        
        record_id
    }

    pub fn get_borrow_record(&self, record_id: &str) -> Option<&BorrowRecord> {
        self.records.get(record_id)
    }

    pub fn get_user_borrows(&self, user_id: &str) -> Vec<&BorrowRecord> {
        self.records
            .values()
            .filter(|record| record.user_id() == user_id)
            .collect()
    }

    pub fn get_active_borrows(&self) -> Vec<&BorrowRecord> {
        self.records
            .values()
            .filter(|record| record.return_date().is_none())
            .collect()
    }

    pub fn check_overdue_records(&self) {
        for record in self.records.values() {
            if record.is_overdue() {
                println!("记录 {} 已逾期", record.id());
            }
        }
    }
}