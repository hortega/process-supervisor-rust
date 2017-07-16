#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;

#[cfg(test)]
mod tests;
mod command_repository;

use command_repository::{Command, CommandRepository};
use std::sync::RwLock;
use rocket::State;
use rocket_contrib::{Json, Value};

#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}

#[put("/commands", format = "application/json", data = "<command>")]
pub fn upsert_command(repo: State<RwLock<CommandRepository>>, command: Json<Command>) -> Result<String, String> {
    let mut repo = repo.write().unwrap();
    let command_cloned = command.into_inner().clone();
    let id = repo.store(command_cloned);
    Ok(id.to_string())
}

#[get("/commands")]
pub fn read_commands(repo: State<RwLock<CommandRepository>>) -> Json<Value> {
    let res = &*repo.read().unwrap();
    Json(json!(res))
}

fn main() {
    rocket::ignite()
        .manage(RwLock::new(CommandRepository::new()))
        .mount("/", routes![ping, upsert_command, read_commands])
        .launch();
}
