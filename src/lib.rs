#[derive(PartialEq)]
pub enum Command {
    Quit,
}

pub fn handle_commands(commands: Vec<Command>) -> Vec<String> {
    let result = vec![];

    for command in commands {
        if command == Command::Quit {
            return result;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{handle_commands, Command};

    #[test]
    fn quit_command() {
        let result = handle_commands(vec![Command::Quit]);

        assert_eq!(result, Vec::<String>::new())
    }
}
