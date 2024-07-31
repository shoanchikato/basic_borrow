use crate::model::person::Person;

pub trait Repository: Send + Sync {
    fn add(&mut self, new_person: Box<Person>);
    fn remove(&mut self, id: i32);
    fn get_all(&self) -> Vec<Person>;
    fn get_one(&self, id: i32) -> Option<Person>;
    fn update(&mut self, id: i32, new_person: Box<Person>) -> Option<Person>;
}

pub struct Repo {
    people: Vec<Person>,
}

impl Repo {
    pub fn new() -> Self {
        Self { people: vec![] }
    }
}

impl Repository for Repo {
    fn add(&mut self, new_person: Box<Person>) {
        let mut person = new_person.to_owned();
        let id = self.people.len() + 1;
        person.id = id as i32;

        self.people.push(*person);
    }

    fn remove(&mut self, id: i32) {
        let index = self.people.iter_mut().position(|person| person.id == id);
        match index {
            Some(index) => {
                self.people.remove(index);
            }
            None => eprintln!("Person not found with id"),
        }
    }

    fn get_all(&self) -> Vec<Person> {
        self.people.to_owned()
    }

    fn get_one(&self, id: i32) -> Option<Person> {
        let person = self.people.iter().find(|person| person.id == id);

        person.cloned()
    }

    fn update(&mut self, id: i32, new_person: Box<Person>) -> Option<Person> {
        let index = self.people.iter_mut().position(|person| person.id == id);
        match index {
            Some(index) => {
                if let Some(person) = self.people.get_mut(index) {
                    person.age = new_person.age;
                    let name = new_person.name;
                    person.name = name;

                    Some(person.to_owned())
                } else {
                    eprintln!("Person not found with id");
                    None
                }
            }
            None => {
                eprintln!("Person not found with id");
                None
            }
        }
    }
}
