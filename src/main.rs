#![allow(dead_code)]

use basic_borrow::interface::app::{Interface, Interfacer};
use basic_borrow::model::person::Person;
use basic_borrow::repo::people::Repo;
use basic_borrow::service::people::Service;
fn main() {
    let mut repo = Repo::new();
    let mut service = Service::new(&mut repo);
    let mut interface = Interface::new(&mut service);

    let people = vec![Person::new("John", 32), Person::new("Susan", 22)];

    println!("Add");
    people.iter().for_each(|person| interface.add(person));
    interface.print();

    println!("Update");
    let new_person = Person::new("James", 33);
    interface.update(1, &new_person);
    interface.print();

    println!("Remove");
    interface.remove(1);
    interface.print();
}
