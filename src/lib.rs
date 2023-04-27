use std::collections::HashMap;

#[derive(PartialEq)]
pub enum Command {
    Quit,
    Print(String),
    Add(String, usize),
}

pub fn handle_commands(commands: Vec<Command>) -> Vec<String> {
    let mut outputs = vec![];
    let mut registers: HashMap<String, usize> = HashMap::new();

    for command in commands {
        match command {
            Command::Quit => return outputs,
            Command::Print(register_name) => {
                let output = registers
                .get(&register_name)
                .map(|value| value.to_string())
                .unwrap_or_else(|| format!("Register {register_name} was not initialized when command print was executed on row 1."));

                outputs.push(output);
            }
            Command::Add(register_name, value) => {
                registers.insert(register_name, value);
            }
        };
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

    #[test]
    fn a_add_1() {
        let result = handle_commands(vec![
            Command::Add("A".to_string(), 1),
            Command::Print("A".to_string()),
            Command::Quit,
        ]);

        assert_eq!(result, vec!["1".to_string()])
    }
}
