#![allow(dead_code)]

use axum::routing::Router;
use std::sync::Arc;

use basic_borrow::interface::app::{Interface, Interfacer};
use basic_borrow::model::person::Person;
use basic_borrow::repo::people::Repo;
use basic_borrow::route;
use basic_borrow::service::people::Service;

#[tokio::main]
async fn main() {
    let repo = Repo::new();
    let service = Service::new(Box::new(repo));
    let mut interface = Interface::new(Box::new(service));

    let people = [Person::new("John", 32),
        Person::new("Jane", 22),
        Person::new("James", 25)];

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

    let people_route = route::people::new(Arc::new(interface));

    let app = Router::new().nest("/people", people_route);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start server");
    println!("Running app..");
    axum::serve(listener, app).await.unwrap();
}
