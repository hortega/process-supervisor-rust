use std::collections::HashMap;
use std::process::Child;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandMetadata {
    pub command: Vec<String>,
    pub cwd: String,
    pub state: String
}

#[derive(Debug)]
pub struct Command {
    pub command_metadata: CommandMetadata,
    pub process: Option<Child>
}

#[derive(Debug)]
pub struct CommandRepository {
    commands: HashMap<String, Command>
}

impl CommandRepository {
    pub fn new() -> CommandRepository {
        CommandRepository {
            commands: HashMap::new(),
        }
    }

    pub fn lookup(&mut self, command_name: String) -> Option<&mut Command> {
        self.commands.get_mut(&command_name)
    }

    pub fn store(&mut self, command_metadata: CommandMetadata) -> String {
        let command_name = match command_metadata.command.first() {
            Some(cmd) => cmd.clone().to_string(),
            None => return "".to_string()
        };
        if let Some(command) = self.lookup(command_name.clone()) {
            command.command_metadata = command_metadata;
            return command_name;
        }

        let command = Command { command_metadata: command_metadata, process: None};
        self.commands.insert(command_name.to_string().clone(), command);
        command_name
    }

    pub fn retrieve(&self) -> Vec<CommandMetadata> {
        let mut res = vec!();
        for command in self.commands.values() {
            res.push(command.command_metadata.clone());
        }
        res
    }

}
