#![allow(dead_code)]

use basic_borrow::interface::interface::{Interface, Interfacer};
use basic_borrow::model::person::Person;
use basic_borrow::repo::people::Repo;
use basic_borrow::service::people::Service;
fn main() {
    let repo = Repo::new();
    let service = Service::new(Box::new(repo));
    let mut interface = Interface::new(Box::new(service));

    let people = vec![
        Person::new("John", 32),
        Person::new("Jane", 22),
        Person::new("James", 25),
    ];

    println!("Add");
    people
        .iter()
        .for_each(|person| interface.add(Box::new(person.to_owned())));
    interface.print();

    println!("Update");
    let new_person = Person::new("James", 33);
    interface.update(1, Box::new(new_person));
    interface.print();

    println!("Remove");
    interface.remove(1);
    interface.print();
}
