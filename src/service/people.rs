use crate::{model::person::Person, repo::people::Repository};

pub trait Servicer: Send + Sync {
    fn add(&mut self, new_person: Box<Person>);
    fn remove(&mut self, id: i32);
    fn get_all(&self) -> Vec<Person>;
    fn get_one(&self, id: i32) -> Option<Person>;
    fn update(&mut self, id: i32, new_person: Box<Person>) -> Option<Person>;
}

pub struct Service {
    repo: Box<dyn Repository>,
}

impl Service {
    pub fn new(repo: Box<dyn Repository>) -> Self {
        Self { repo }
    }
}

impl Servicer for Service {
    fn add(&mut self, new_person: Box<Person>) {
        self.repo.add(new_person);
    }

    fn remove(&mut self, id: i32) {
        self.repo.remove(id);
    }

    fn get_all(&self) -> Vec<Person> {
        self.repo.get_all()
    }

    fn get_one(&self, id: i32) -> Option<Person> {
        self.repo.get_one(id)
    }

    fn update(&mut self, id: i32, new_person: Box<Person>) -> Option<Person> {
        self.repo.update(id, new_person)
    }
}
