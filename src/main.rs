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
use std::process::Command;

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

fn supervisor(command_repository_clone: &Arc<RwLock<CommandRepository>>) {
    let mut write_repo_wrapped = command_repository_clone.write();
    let mut write_repo = write_repo_wrapped.unwrap();
    let command_names = write_repo.read_command_names();
    println!("In loop. Commands: {:?}", command_names);
    for command_name in command_names {
        let command = write_repo.take_from_name(&command_name);
        if command.is_some() {
            let mut command = command.unwrap();
            println!("****  Command state {:?}", command.command_metadata.state);
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
                        println!("**** Child process exists in model, is stopped and should be running");
                        // Process is not running but should be running
                        let full_command_name = CommandRepository::
                        build_fully_qualified_command(&command_name, &command.command_metadata);
                        println!("**** Starting command {:?}", full_command_name);
                        let args = CommandRepository::get_arguments(&command.command_metadata);
                        process = Command::new(full_command_name)
                            .args(args)
                            .spawn()
                            .expect("failed to execute child");

                        new_command.process = Some(process);
                    } else if !is_stopped && command.command_metadata.state == "stopped" {
                        println!("**** Child process exists in model, is running and should be stopped");
                        // Process is running but shouldn't be
                        process.kill().unwrap();
                        new_command.process = None;
                    } else {
                        println!("**** Child process exists in model, no action is needed {:?}", process.id());
                        new_command.process = Some(process);
                    }
                    write_repo.commands.insert(new_command);
                },
                None => {
                    println!("**** Child process DOES NOT exists in model");
                    // Process is not running
                    if command.command_metadata.state == "running" {
                        println!("**** Child process DOES NOT exists in model and should be running");
                        // But should be running
                        let full_command_name = CommandRepository::
                        build_fully_qualified_command(&command_name, &command.command_metadata);
                        let args = CommandRepository::get_arguments(&command.command_metadata);
                        let process = Command::new(full_command_name)
                            .args(args)
                            .spawn()
                            .expect("failed to execute child");
                        println!("**** New child command id {:?}", process.id());
                        new_command.process = Some(process);
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
