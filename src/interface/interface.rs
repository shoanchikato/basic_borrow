use crate::model::person::Person;
use crate::service::people::Servicer;

pub trait Interfacer {
    fn add(&mut self, new_person: Box<Person>);
    fn remove(&mut self, id: i32);
    fn get_all(&self) -> Vec<Person>;
    fn get_one(&self, id: i32) -> Option<Person>;
    fn update(&mut self, id: i32, new_person: Box<Person>) -> Option<Person>;
    fn print(&self);
}

pub struct Interface {
    service: Box<dyn Servicer>,
}

impl Interface {
    pub fn new(service: Box<dyn Servicer>) -> Self {
        Self { service }
    }
}

impl Interfacer for Interface {
    fn add(&mut self, new_person: Box<Person>) {
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

    fn update(&mut self, id: i32, new_person: Box<Person>) -> Option<Person> {
        self.service.update(id, new_person)
    }

    fn print(&self) {
        self.service
            .get_all()
            .iter()
            .for_each(|person| println!("{person}"));
    }
}
