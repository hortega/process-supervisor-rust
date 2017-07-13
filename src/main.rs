#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::JSON;


#[derive(Serialize, Deserialize)]
struct CommandRequest {
    command: Vec<String>,
    cwd: String,
    state: String
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[put("/commands", format = "application/json", data = "<command_request>")]
fn upsert_command(command_request: JSON<CommandRequest>) -> JSON<CommandRequest> {
     command_request
}

fn main() {
    rocket::ignite()
        .mount("/", routes![ping, upsert_command])
        .launch();
}
