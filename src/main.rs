use std::error::Error;

mod models;
mod services;

use models::{Student, Book, BorrowRecord};
use services::{UserService, BookService, BorrowService};

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== 高校图书借阅系统 ===");
    
    // 初始化服务
    let mut user_service = UserService::new();
    let mut book_service = BookService::new();
    let mut borrow_service = BorrowService::new();
    
    // 添加示例用户
    let student = Student::new(
        "1001".to_string(),
        "张三".to_string(),
    );
    user_service.add_user(student);
    
    // 添加示例图书
    let book1 = Book::new(
        "978-7-115-41632-2".to_string(),
        "Rust程序设计".to_string(),
        "Steve Klabnik".to_string(),
        "人民邮电出版社".to_string(),
    );
    book_service.add_book(book1);
    
    let book2 = Book::new(
        "978-7-302-33064-6".to_string(),
        "数据结构与算法".to_string(),
        "Robert Sedgewick".to_string(),
        "清华大学出版社".to_string(),
    );
    book_service.add_book(book2);
    
    println!("系统初始化完成！");
    println!("用户数量: {}", user_service.get_user_count());
    println!("图书数量: {}", book_service.get_book_count());
    
    Ok(())
}