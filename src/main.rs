use quirky_math::{handle_commands, Command};

fn main() {
    let commands = vec![Command::Print("A".to_string()), Command::Quit];

    let outputs = handle_commands(commands);

    for output in outputs {
        println!("{output}");
    }
}
