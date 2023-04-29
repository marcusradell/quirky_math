use quirky_math::{handle_commands, Command, ValueEnum};

fn main() {
    let commands_536 = vec![
        Command::Add("A".to_string(), ValueEnum::Primitive(2)),
        Command::Add("A".to_string(), quirky_math::ValueEnum::Primitive(3)),
        Command::Print("A".to_string()),
        Command::Add("B".to_string(), quirky_math::ValueEnum::Primitive(5)),
        Command::Subtract("B".to_string(), 2),
        Command::Print("B".to_string()),
        Command::Add("A".to_string(), quirky_math::ValueEnum::Primitive(1)),
        Command::Print("A".to_string()),
        Command::Quit,
    ];

    let outputs = handle_commands(commands_536);

    for output in outputs {
        println!("{output}");
    }
}
