pub trait User {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn borrow_limit(&self) -> u32;
}

#[derive(Debug, Clone)]
pub struct Student {
    pub student_id: String,
    pub name: String,
}

impl Student {
    pub fn new(student_id: String, name: String) -> Self {
        Self {
            student_id,
            name,
        }
    }
}

impl User for Student {
    fn id(&self) -> &str {
        &self.student_id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn borrow_limit(&self) -> u32 {
        5
    }
}

#[derive(Debug, Clone)]
pub struct Teacher {
    pub teacher_id: String,
    pub name: String,
}

impl Teacher {
    pub fn new(teacher_id: String, name: String) -> Self {
        Self {
            teacher_id,
            name,
        }
    }
}

impl User for Teacher {
    fn id(&self) -> &str {
        &self.teacher_id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn borrow_limit(&self) -> u32 {
        10
    }
}