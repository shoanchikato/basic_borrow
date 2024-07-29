use crate::model::person::Person;
use crate::service::people::Servicer;

pub trait Interfacer<'a> {
    fn add(&mut self, new_person: &'a Person);
    fn remove(&mut self, id: i32);
    fn get_all(&self) -> Vec<Person>;
    fn get_one(&self, id: i32) -> Option<Person>;
    fn update(&mut self, id: i32, new_person: &'a Person) -> Option<Person>;
    fn print(&self);
}

pub struct Interface<'a> {
    service: &'a mut dyn Servicer<'a>,
}

impl<'a> Interface<'a> {
    pub fn new(service: &'a mut dyn Servicer<'a>) -> Self {
        Self { service }
    }
}

impl<'a> Interfacer<'a> for Interface<'a> {
    fn add(&mut self, new_person: &'a Person) {
        self.service.add(new_person);
    }

    fn remove(&mut self, id: i32) {
        self.service.remove(id);
    }

    fn get_all(&self) -> Vec<Person> {
        self.service.get_all()
    }

    fn get_one(&self, id: i32) -> Option<Person> {
        self.service.get_one(id)
    }

    fn update(&mut self, id: i32, new_person: &'a Person) -> Option<Person> {
        self.service.update(id, new_person)
    }

    fn print(&self) {
        self.service
            .get_all()
            .iter()
            .for_each(|person| println!("{person}"));
    }
}
