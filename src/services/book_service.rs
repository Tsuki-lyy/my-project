use std::collections::HashMap;

use crate::models::Book;

pub struct BookService {
    books: HashMap<String, Book>,
}

impl BookService {
    pub fn new() -> Self {
        Self {
            books: HashMap::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.insert(book.isbn().to_string(), book);
    }

    pub fn get_book(&self, book_isbn: &str) -> Option<&Book> {
        self.books.get(book_isbn)
    }

    pub fn search_books(&self, keyword: &str) -> Vec<&Book> {
        self.books
            .values()
            .filter(|book| {
                book.title().contains(keyword)
                    || book.author().contains(keyword)
                    || book.isbn().contains(keyword)
                    || book.publisher().contains(keyword)
            })
            .collect()
    }

    pub fn get_book_count(&self) -> usize {
        self.books.len()
    }

    pub fn get_available_books(&self) -> Vec<&Book> {
        self.books
            .values()
            .filter(|book| book.available())
            .collect()
    }
}