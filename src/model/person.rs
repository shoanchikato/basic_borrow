use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct Person<'a> {
    pub id: i32,
    pub name: &'a str,
    pub age: i32,
}

impl<'a> Person<'a> {
    pub fn new(name: &'a str, age: i32) -> Self {
        Self { id: 0, name, age }
    }
}

impl<'a> Display for Person<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&format!(
            "Person {{ id: {}, name: \"{}\", age: {} }}",
            &self.id, &self.name, &self.age
        ))
    }
}
