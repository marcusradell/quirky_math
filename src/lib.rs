use std::{cell::Cell, collections::HashMap};

#[derive(PartialEq, Debug)]
pub struct Value {
    pub number: Cell<isize>,
    pub next: Option<Box<Value>>,
}

impl Value {
    pub fn get_total(&self) -> isize {
        match self.next.as_ref() {
            Some(value) => value.get_total() + self.number.get(),
            None => self.number.get(),
        }
    }
}

#[derive(PartialEq)]
pub enum Command {
    Quit,
    Print(String),
    Add(String, isize),
    LazyAdd(String, Value),
    Subtract(String, isize),
    Multiply(String, isize),
}

pub fn interior_mutability_lab() {
    let mut lazy_register: HashMap<String, &Box<Value>> = HashMap::new();

    let mut a_value = Box::new(Value {
        number: Cell::new(0),
        next: None,
    });

    a_value.number.set(a_value.number.get() + 1);

    let mut b_value = Box::new(Value {
        next: None,
        number: Cell::new(10),
    });

    a_value.next = Some(b_value);

    lazy_register.insert("a".to_string(), &a_value);

    println!("{lazy_register:?}");

    let a_total = a_value.get_total();

    println!("{a_total}");
}

pub fn handle_commands(commands: Vec<Command>) -> Vec<String> {
    let mut outputs = vec![];
    let mut registers: HashMap<String, isize> = HashMap::new();

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
                let current_value = registers.get(register_name).unwrap_or(&0);
                registers.insert(register_name.clone(), *current_value + value);
            }
            Command::Subtract(register_name, value) => {
                let current_value = registers.get(register_name).unwrap_or(&0);
                registers.insert(register_name.clone(), current_value - value);
            }
            Command::Multiply(register_name, value) => {
                let current_value = registers.get(register_name).unwrap_or(&0);
                registers.insert(register_name.clone(), current_value * value);
            }
            Command::LazyAdd(target_register_name, source_register_name) => {
                let current_value = registers.get(target_register_name).unwrap_or(&0);
                registers.insert(target_register_name.clone(), current_value + 10);
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

    // #[test]
    // fn register_as_value() {
    //     let result = handle_commands(vec![
    //         Command::Add("A".to_string(), Value::Primitive(10)),
    //         Command::Add("B".to_string(), Value::Lazy("A".to_string())),
    //         Command::Add("B".to_string(), Value::Primitive(1)),
    //         Command::Print("B".to_string()),
    //     ]);

    //     assert_eq!(result, vec!["11".to_string()]);
    // }
}
