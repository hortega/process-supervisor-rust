use std::collections::HashSet;
use std::process::Child;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandMetadata {
    pub command: Vec<String>,
    pub cwd: String,
    pub state: String
}

#[derive(Debug)]
pub struct Command {
    pub command_name: String,
    pub command_metadata: CommandMetadata,
    pub process: Option<Child>
}

impl Hash for Command {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.command_name.hash(state);
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Command) -> bool {
        self.command_name == other.command_name
    }
}
impl Eq for Command {}

#[derive(Debug)]
pub struct CommandRepository {
    pub commands: HashSet<Command>
}

impl CommandRepository {
    pub fn new() -> CommandRepository {
        CommandRepository {
            commands: HashSet::new(),
        }
    }

    pub fn store(&mut self, command_metadata: CommandMetadata) -> String {
        let command_name = match command_metadata.command.first() {
            Some(cmd) => cmd.clone().to_string(),
            None => return "".to_string()
        };
        println!("/// metadata {:?}", command_metadata);
        let mut command = Command {
            command_name: command_name.clone(),
            command_metadata: command_metadata,
            process: None};
        println!("/// metadata {:?}", command);
        if let Some(stored_command) = self.commands.take(&command) {
            command.process = stored_command.process;
        }

        self.commands.insert(command);
        command_name
    }

    pub fn read_metadata(&self) -> Vec<CommandMetadata> {
        let mut res = vec!();
        for command in self.commands.iter() {
            res.push(command.command_metadata.clone());
        }
        res
    }

    pub fn read_command_names(&self) -> Vec<String> {
        let mut res = vec!();
        for command in self.commands.iter() {
            res.push(command.command_name.clone());
        }
        res
    }

    pub fn take_from_name(&mut self, command_name: &String) -> Option<Command> {
        let command = Command {
            command_name: command_name.clone(),
            command_metadata: CommandMetadata {
                command: vec!(), cwd: "".to_string(), state:"".to_string()
            },
            process: None};
        self.commands.take(&command)
    }

    pub fn build_fully_qualified_command(command_name: &String, command_metadata: &CommandMetadata) -> String {
        format!("{}/{}", command_metadata.cwd.clone(), command_name.clone().to_string())
    }

    pub fn get_arguments(command_metadata: &CommandMetadata) -> Vec<String> {
        command_metadata.command[1..].to_vec()
    }
}
