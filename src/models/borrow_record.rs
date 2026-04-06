use chrono::{NaiveDate, Duration};

#[derive(Debug, Clone)]
pub struct BorrowRecord {
    id: String,
    user_id: String,
    book_isbn: String,
    borrow_date: NaiveDate,
    due_date: NaiveDate,
    return_date: Option<NaiveDate>,
}

impl BorrowRecord {
    pub fn new(id: String, user_id: String, book_isbn: String) -> Self {
        let borrow_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let due_date = borrow_date + Duration::days(30);

        Self {
            id,
            user_id,
            book_isbn,
            borrow_date,
            due_date,
            return_date: None,
        }
    }

    pub fn is_overdue(&self) -> bool {
        let today = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        today > self.due_date && self.return_date.is_none()
    }

    pub fn calculate_fine(&self) -> f64 {
        if !self.is_overdue() {
            return 0.0;
        }

        let today = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let days_overdue = (today - self.due_date).num_days();
        days_overdue as f64
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn book_isbn(&self) -> &str {
        &self.book_isbn
    }

    pub fn borrow_date(&self) -> &NaiveDate {
        &self.borrow_date
    }

    pub fn due_date(&self) -> &NaiveDate {
        &self.due_date
    }

    pub fn return_date(&self) -> &Option<NaiveDate> {
        &self.return_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_is_overdue() {
        // 测试未逾期的情况：今天是2024-01-15，due_date是2024-01-31
        let record = BorrowRecord::new(
            "BR1".to_string(),
            "user1".to_string(),
            "978-7-115-41632-2".to_string(),
        );
        
        // 未逾期
        assert!(!record.is_overdue());
        
        // 已归还的情况
        let mut returned_record = record;
        returned_record.return_date = Some(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap());
        assert!(!returned_record.is_overdue());
    }

    #[test]
    fn test_is_overdue_true() {
        // 测试已逾期的情况：创建一个due_date在今天之前的记录
        let mut record = BorrowRecord::new(
            "BR2".to_string(),
            "user2".to_string(),
            "978-7-302-33064-6".to_string(),
        );
        
        // 修改due_date为今天之前
        record.due_date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        
        // 已逾期且未归还
        assert!(record.is_overdue());
    }

    #[test]
    fn test_calculate_fine() {
        // 测试未逾期的情况
        let record1 = BorrowRecord::new(
            "BR3".to_string(),
            "user3".to_string(),
            "978-7-115-41632-2".to_string(),
        );
        assert_eq!(record1.calculate_fine(), 0.0);
        
        // 测试已逾期的情况：逾期5天
        let mut record2 = BorrowRecord::new(
            "BR4".to_string(),
            "user4".to_string(),
            "978-7-302-33064-6".to_string(),
        );
        record2.due_date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        
        // 今天是2024-01-15，逾期5天，罚款5元
        assert_eq!(record2.calculate_fine(), 5.0);
        
        // 测试已归还的情况
        let mut record3 = record2;
        record3.return_date = Some(NaiveDate::from_ymd_opt(2024, 1, 12).unwrap());
        assert_eq!(record3.calculate_fine(), 0.0);
    }
}