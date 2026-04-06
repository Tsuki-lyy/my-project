#[derive(Debug, Clone)]
pub struct Book {
    isbn: String,
    title: String,
    author: String,
    publisher: String,
    available: bool,
}

impl Book {
    pub fn new(isbn: String, title: String, author: String, publisher: String) -> Self {
        Self {
            isbn,
            title,
            author,
            publisher,
            available: true,
        }
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