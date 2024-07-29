use crate::{model::person::Person, repo::people::Repository};

pub trait Servicer<'a> {
  fn add(&mut self, new_person: &'a Person);
  fn remove(&mut self, id: i32);
  fn get_all(&self) -> Vec<Person>;
  fn get_one(&self, id: i32) -> Option<Person>;
  fn update(&mut self, id: i32, new_person: &'a Person) -> Option<Person>;
}

pub struct Service<'a> {
  repo: &'a mut dyn Repository<'a>,
}

impl <'a> Service<'a> {
  pub fn new(repo: &'a mut dyn Repository<'a>) -> Self {
    Self { repo }
  }
}

impl <'a> Servicer<'a> for Service <'a> {
    fn add(&mut self, new_person: &'a Person) {
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

    fn update(&mut self, id: i32, new_person: &'a Person) -> Option<Person> {
        self.repo.update(id, new_person)
    }
}