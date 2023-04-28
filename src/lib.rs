use std::collections::HashMap;

#[derive(PartialEq)]
pub enum Command {
    Quit,
    Print(String),
    Add(String, usize),
    Subtract(String, usize),
}

pub fn handle_commands(commands: Vec<Command>) -> Vec<String> {
    let mut outputs = vec![];
    let mut registers: HashMap<String, usize> = HashMap::new();

    for (index, command) in commands.iter().enumerate() {
        match command {
            Command::Quit => return outputs,
            Command::Print(register_name) => {
                let output = registers
                .get(register_name)
                .map(|value| value.to_string())
                .unwrap_or_else(|| format!("Register {register_name} was uninitialized when command PRINT with index {index} was executed."));

                outputs.push(output);
            }
            Command::Add(register_name, value) => {
                registers.insert(register_name.clone(), *value);
            }
            Command::Subtract(register_name, value) => {
                let current_value = registers.get(register_name).unwrap_or(&0);
                registers.insert(register_name.clone(), current_value - value);
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
            vec!["Register A was uninitialized when command PRINT with index 0 was executed."]
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

    #[test]
    fn b_add_5_subtract_2() {
        let result = handle_commands(vec![
            Command::Add("B".to_string(), 5),
            Command::Subtract("B".to_string(), 2),
            Command::Print("B".to_string()),
            Command::Quit,
        ]);

        assert_eq!(result, vec!["3".to_string()])
    }
}
