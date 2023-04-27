use quirky_math::{handle_commands, Command};

fn main() {
    let commands = vec![Command::Quit];

    let output = handle_commands(commands);

    println!("{output:?}");
}
