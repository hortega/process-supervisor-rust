use super::rocket_with_repository;
use rocket::local::Client;
use rocket::http::{Status, ContentType};


#[test]
fn test_ping() {
    let client = Client::new(rocket_with_repository()).expect("valid rocket instance");
    let mut response = client.get("/ping").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("pong".into()));
}

#[test]
fn test_commands_get_put() {
    let client = Client::new(rocket_with_repository()).expect("valid rocket instance");

    // assert there are no commands
    let mut response = client.get("/commands").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("{\"commands\":{}}".into()));

    // send a command
    let mut response = client.put("/commands")
        .header(ContentType::JSON)
        .body(r#"{
                    "command": ["service.sh", "arg1"],
                    "cwd": "/path",
                    "state": "running"
              }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("service.sh".into()));

    // assert the command was stored
    let mut response = client.get("/commands").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body = response.body().unwrap().into_string().unwrap();
    assert!(body.contains("service.sh"));
    assert!(body.contains("arg1"));
    assert!(body.contains("/path"));
}
