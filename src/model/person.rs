use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

impl Person {
    pub fn new(name: &str, age: i32) -> Self {
        Self {
            id: 0,
            name: name.into(),
            age,
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&format!(
            "Person {{ id: {}, name: \"{}\", age: {} }}",
            &self.id, &self.name, &self.age
        ))
    }
}
