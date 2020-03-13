pub mod parser;
pub struct Command {
    pub key: String,
    pub args: Vec<String>,
}

pub struct CommandRepo {
    pub commands: Vec<Command>,
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
    pub fn init_commands(&mut self) {
        self.commands.push(Command { key: String::from("test"), args: Vec::new()});
        self.commands.push(Command { key: String::from("n"), args: Vec::new()});
    }
}

pub fn parse_command(cmd: String, repo: &CommandRepo) {
    let key = parser::parse_command_key(&cmd).unwrap();
    let args = parser::parse_command_args(&cmd).unwrap();
    println!("key: {}\nargs: {:?}", key, args);
    for c in repo.commands.iter() {
        if c.key == key {
            println!("Found a command!")
        } else {
            println!("Could not find a command")
        }
    }
}
