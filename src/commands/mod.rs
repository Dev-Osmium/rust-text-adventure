pub mod parser;
pub struct Command {
    pub key: String,
    pub args: Vec<String>,
}

pub struct CommandRepo {
    pub commands: Vec<&'static Command>,
}

impl Command {
    pub fn new(key: String, args: Vec<String>) -> Command {
        let command: Command = Command { key, args };
        command
    }
}
impl CommandRepo {
    pub fn new() -> CommandRepo {
        let repo: CommandRepo = CommandRepo { commands: Vec::new() };
        repo
    }
}

pub fn parse_command(cmd: String) {
    let key = parser::parse_command_key(&cmd);
    let args = parser::parse_command_args(&cmd);
}
