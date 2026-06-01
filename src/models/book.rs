#[derive(Debug, Clone, PartialEq)]
pub enum BookCategory {
    Fiction,
    Science,
    Technology,
    History,
    Education,
    Other,
}

#[derive(Debug, Clone)]
pub struct Book {
    isbn: String,
    title: String,
    author: String,
    publisher: String,
    category: BookCategory,
    available: bool,
}

impl Book {
    pub fn new(isbn: String, title: String, author: String, publisher: String, category: BookCategory) -> Self {
        Self {
            isbn,
            title,
            author,
            publisher,
            category,
            available: true,
        }
    }

    pub fn category(&self) -> &BookCategory {
        &self.category
    }

    pub fn borrow(&mut self) -> Result<(), String> {
        if self.available {
            self.available = false;
            Ok(())
        } else {
            Err("图书不可借".to_string())
        }
    }

    pub fn return_book(&mut self) {
        self.available = true;
    }

    pub fn isbn(&self) -> &str {
        &self.isbn
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn publisher(&self) -> &str {
        &self.publisher
    }

    pub fn available(&self) -> bool {
        self.available
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_with_category() {
        let book = Book::new(
            "978-7-115-41632-2".to_string(),
            "Rust Programming".to_string(),
            "Steve Klabnik".to_string(),
            "O'Reilly".to_string(),
            BookCategory::Technology,
        );
        assert_eq!(book.category(), &BookCategory::Technology);
    }
}