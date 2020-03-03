pub fn parse_command_key(cmd: &String) -> Result<String, String> {
    let key = String::from(cmd.split_whitespace().next().unwrap());
    println!("{}", key);
    Ok(String::from(key))
}

pub fn parse_command_args(cmd: &String) -> Result<Vec<String>, String> {
    let iter = cmd.split_whitespace();
    let mut args = Vec::new();
    for (i, s) in iter.enumerate() {
        if i == 0 {
            continue;
        } else {
            args.push(String::from(s));
        }
    }
    Ok(args)
}
