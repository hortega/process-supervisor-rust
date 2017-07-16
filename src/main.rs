#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;
extern crate futures;


#[cfg(test)]
mod tests;
mod command_repository;

use command_repository::{CommandMetadata, CommandRepository};
use std::sync::{Arc, RwLock};
use std::thread;
use rocket::State;
use rocket_contrib::{Json, Value};
use std::time::Duration;

#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}

// TODO: Accept an array of commands
#[put("/commands", format = "application/json", data = "<command_metadata>")]
pub fn upsert_command(repo: State<Arc<RwLock<CommandRepository>>>, command_metadata: Json<CommandMetadata>) -> Result<String, String> {
    let mut repo = repo.write().unwrap();
    let command_cloned = command_metadata.into_inner().clone();
    let id = repo.store(command_cloned);
    Ok(id.to_string())
}

#[get("/commands")]
pub fn read_commands(repo: State<Arc<RwLock<CommandRepository>>>) -> Json<Value> {
    let read = &*repo.read().unwrap().retrieve();

    Json(json!(read))
}

fn rocket(command_repository: Arc<RwLock<CommandRepository>>) -> rocket::Rocket {
    rocket::ignite()
        .manage(command_repository)
        .mount("/", routes![ping, upsert_command, read_commands])
}

fn rocket_with_repository() -> rocket::Rocket {
    rocket(Arc::new(RwLock::new(CommandRepository::new())))
}

fn main() {
    let command_repository = Arc::new(RwLock::new(CommandRepository::new()));
    let command_repository_clone = command_repository.clone();
    thread::spawn(move || {
        loop {

            thread::sleep(Duration::from_millis(4000))
        }
    });

    rocket(command_repository).launch();
}
