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
use std::process::{Command, Child};

#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}

#[put("/commands", format = "application/json", data = "<command_metadata>")]
pub fn upsert_command(repo: State<Arc<RwLock<CommandRepository>>>, command_metadata: Json<CommandMetadata>) -> Result<String, String> {
    let mut repo = repo.write().unwrap();
    let command_cloned = command_metadata.into_inner().clone();
    let id = repo.store(command_cloned);
    Ok(id.to_string())
}

#[get("/commands")]
pub fn read_commands(repo: State<Arc<RwLock<CommandRepository>>>) -> Json<Value> {
    let read = &*repo.read().unwrap().read_metadata();

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

fn start_command(command_name: &String, command_metadata: &CommandMetadata) -> Child {
    let full_command_name = CommandRepository::
        build_fully_qualified_command(command_name, command_metadata);
    let args = CommandRepository::get_arguments(command_metadata);
    let process = Command::new(full_command_name)
        .args(args)
        .spawn()
        .expect("failed to execute child");
    println!("New command started, ID: {:?}", process.id());
    process
}

fn supervisor(command_repository_clone: &Arc<RwLock<CommandRepository>>) {
    let mut write_repo = command_repository_clone.write().unwrap();
    let command_names = write_repo.read_command_names();
    for command_name in command_names {
        let command = write_repo.take_from_name(&command_name);
        if command.is_some() {
            let command = command.unwrap();
            let mut new_command = command_repository::Command {
                command_name: command_name.clone(),
                command_metadata: command.command_metadata.clone(),
                process: None
            };
            match command.process {
                Some(mut process) => {
                    let is_stopped = match process.try_wait() {
                        Ok(Some(_)) => true,
                        Ok(None) => false,
                        Err(_) => true
                    };
                    if is_stopped && command.command_metadata.state == "running" {
                        // Process is not running but should be running
                        new_command.process = Some(start_command(&command_name, &command.command_metadata));
                    } else if !is_stopped && command.command_metadata.state == "stopped" {
                        // Process is running but shouldn't be
                        process.kill().unwrap();
                        new_command.process = None;
                    } else {
                        // The process is in the same state as required by the model, so no
                        // action is required
                        new_command.process = Some(process);
                    }
                    write_repo.commands.insert(new_command);
                },
                None => {
                    // Process is not running
                    if command.command_metadata.state == "running" {
                        // But should be running
                        new_command.process = Some(start_command(&command_name, &command.command_metadata));
                    }
                    write_repo.commands.insert(new_command);
                }
            }
        }
    }
}

fn main() {
    let command_repository = Arc::new(RwLock::new(CommandRepository::new()));
    let command_repository_clone = command_repository.clone();
    thread::spawn(move || {
        loop {
            supervisor(&command_repository_clone);
            thread::sleep(Duration::from_millis(1000))
        }
    });

    rocket(command_repository).launch();
}
