use chrono::NaiveDate;
use crate::models::BorrowRecord;

pub trait FineCalculator {
    fn calculate(&self, borrow_record: &BorrowRecord) -> f64;
}

pub struct DefaultFineCalculator;

impl FineCalculator for DefaultFineCalculator {
    fn calculate(&self, borrow_record: &BorrowRecord) -> f64 {
        let today = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        
        if borrow_record.return_date().is_some() {
            return 0.0;
        }
        
        if today <= *borrow_record.due_date() {
            return 0.0;
        }
        
        let days_overdue = (today - *borrow_record.due_date()).num_days();
        days_overdue as f64
    }
}

mod user_service;
mod book_service;
mod borrow_service;

pub use user_service::UserService;
pub use book_service::BookService;
pub use borrow_service::BorrowService;