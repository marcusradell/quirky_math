#[derive(PartialEq)]
pub enum Command {
    Quit,
    Print(String),
}

pub fn handle_commands(commands: Vec<Command>) -> Vec<String> {
    let mut outputs = vec![];

    for command in commands {
        let output = match command {
            Command::Quit => return outputs,
            Command::Print(register_name) => {
                format!("Register {register_name} was not initialized when command print was executed on row 1.")
            }
        };

        outputs.push(output)
    }

    outputs
}

#[cfg(test)]
mod tests {
    use crate::{handle_commands, Command};

    #[test]
    fn no_commands() {
        let result = handle_commands(vec![]);

        assert_eq!(result, Vec::<String>::new())
    }

    #[test]
    fn quit_command() {
        let result = handle_commands(vec![Command::Quit]);

        assert_eq!(result, Vec::<String>::new())
    }

    #[test]
    fn print_command() {
        let result = handle_commands(vec![Command::Print("A".to_string())]);

        assert_eq!(
            result,
            vec!["Register A was not initialized when command print was executed on row 1."]
        )
    }
}
