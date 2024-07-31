use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use std::sync::Arc;

use crate::interface::interface::Interfacer;
use crate::model::person::Person;

async fn get_one(
    Path(id): Path<u64>,
    State(state): State<Arc<dyn Interfacer>>,
) -> Json<Option<Person>> {
    println!("ID {}, was requested", id);
    Json(state.get_one(id as i32))
}

async fn get_all(State(state): State<Arc<dyn Interfacer>>) -> Json<Vec<Person>> {
    Json(state.get_all())
}

pub fn new(state: Arc<dyn Interfacer>) -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/:id", get(get_one))
        .with_state(state)
}
