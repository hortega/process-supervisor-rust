use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Command {
    pub command: Vec<String>,
    pub cwd: String,
    pub state: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandRepository {
    commands: HashMap<String, Command>
}

impl CommandRepository {
    pub fn new() -> CommandRepository {
        CommandRepository {
            commands: HashMap::new(),
        }
    }

    pub fn store(&mut self, command: Command) -> String {
        let command_name = match command.command.first() {
            Some(cmd) => cmd.to_string(),
            None => return "".to_string()
        };
        self.commands.insert(command_name.to_string(), command);
        command_name
    }

    pub fn lookup(&self, command_name: &str) -> Option<&Command> {
        self.commands.get(command_name)
    }
}
