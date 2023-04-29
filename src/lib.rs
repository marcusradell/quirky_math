use std::{cell::RefCell, collections::HashMap, rc::Rc};
use value::Value;

pub mod value;

#[derive(PartialEq)]
pub enum Command {
    Quit,
    Print(String),
    Add(String, isize),
    LazyAdd(String, String),
    Subtract(String, isize),
    Multiply(String, isize),
}

pub fn handle_commands(commands: Vec<Command>) -> Vec<String> {
    let mut outputs = vec![];
    let mut registers: HashMap<String, Rc<RefCell<Value>>> = HashMap::new();

    for (index, command) in commands.iter().enumerate() {
        match command {
            Command::Quit => return outputs,
            Command::Print(register_name) => {
                let output = registers
                .get(register_name)
                .map(|value| value.borrow_mut().get_total().to_string())
                .unwrap_or_else(|| format!("Register {register_name} was uninitialized when command PRINT with index {index} was executed."));

                outputs.push(output);
            }
            Command::Add(register_name, number) => {
                let value = match registers.get(register_name) {
                    Some(value) => Rc::clone(value),
                    None => Value::new_wrapped(),
                };

                let current_number = value.borrow_mut().number.get();

                value.borrow_mut().set_number(current_number + number);

                registers.insert(register_name.clone(), value);
            }
            Command::Subtract(register_name, number) => {
                let value = match registers.get(register_name) {
                    Some(value) => Rc::clone(value),
                    None => Value::new_wrapped(),
                };

                let current_number = value.borrow_mut().number.get();

                value.borrow_mut().set_number(current_number - number);

                registers.insert(register_name.clone(), value);
            }
            Command::Multiply(register_name, number) => {
                let value = match registers.get(register_name) {
                    Some(value) => Rc::clone(value),
                    None => Value::new_wrapped(),
                };

                let current_number = value.borrow_mut().number.get();

                value.borrow_mut().set_number(current_number * number);

                registers.insert(register_name.clone(), value);
            }
            Command::LazyAdd(target_register_name, source_register_name) => {
                let target_value = match registers.get(target_register_name) {
                    Some(value) => Rc::clone(value),
                    None => Value::new_wrapped(),
                };

                let source_value = match registers.get(source_register_name) {
                    Some(value) => Rc::clone(value),
                    None => Value::new_wrapped(),
                };

                target_value.borrow_mut().set_next(source_value);

                registers.insert(target_register_name.clone(), target_value);
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

    #[test]
    fn a_add_1_add_2() {
        let result = handle_commands(vec![
            Command::Add("A".to_string(), 1),
            Command::Add("A".to_string(), 2),
            Command::Print("A".to_string()),
        ]);

        assert_eq!(result, vec!["3".to_string()])
    }

    #[test]
    fn m_add_10_multiply_10() {
        let result = handle_commands(vec![
            Command::Add("M".to_string(), 10),
            Command::Multiply("M".to_string(), 10),
            Command::Print("M".to_string()),
        ]);

        assert_eq!(result, vec!["100".to_string()])
    }

    #[test]
    fn neg_reg_subtract_1() {
        let result = handle_commands(vec![
            Command::Subtract("neg_reg".to_string(), 1),
            Command::Print("neg_reg".to_string()),
        ]);

        assert_eq!(result, vec!["-1"]);
    }

    #[test]
    fn test_1_add_2_multiply_3() {
        let result = handle_commands(vec![
            Command::Add("1".to_string(), 2),
            Command::Multiply("1".to_string(), 3),
            Command::Print("1".to_string()),
        ]);

        assert_eq!(result, vec!["6".to_string()]);
    }

    #[test]
    fn register_as_value() {
        let result = handle_commands(vec![
            Command::Add("A".to_string(), 10),
            Command::LazyAdd("B".to_string(), "A".to_string()),
            Command::Add("B".to_string(), 1),
            Command::Print("B".to_string()),
        ]);

        assert_eq!(result, vec!["11".to_string()]);
    }
}
