// use quirky_math::{handle_commands, value::interior_mutability_lab, Command};

use quirky_math::value_rc::interior_mutability_lab;

fn main() {
    // let commands_536 = vec![
    //     Command::Add("A".to_string(), 2),
    //     Command::Add("A".to_string(), 3),
    //     Command::Print("A".to_string()),
    //     Command::Add("B".to_string(), 5),
    //     Command::Subtract("B".to_string(), 2),
    //     Command::Print("B".to_string()),
    //     Command::Add("A".to_string(), 1),
    //     Command::Print("A".to_string()),
    //     Command::Quit,
    // ];

    // let outputs = handle_commands(commands_536);

    // for output in outputs {
    //     println!("{output}");
    // }

    interior_mutability_lab()
}
