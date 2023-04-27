#[derive(PartialEq)]
enum Command {
    Quit,
}

fn main() {
    let commands = vec![Command::Quit];

    let result = handle_commands(commands);

    println!("{result:?}");
}

fn handle_commands(commands: Vec<Command>) -> Vec<String> {
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
    use crate::handle_commands;

    #[test]
    fn quit_command() {
        let result = handle_commands(vec![]);

        assert_eq!(result, Vec::<String>::new())
    }
}
