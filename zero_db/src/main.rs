mod types;

use crate::types::{MetaCommand, PrepareResult, Statement, VirtualMachine};
use anyhow::{ Result};
use std::io::{self, Write};

#[allow(unreachable_code)]
fn main() -> Result<()> {
    let mut vm = VirtualMachine::new();
    loop {
        let mut input = String::new();
        print!("db > ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        let input = input.trim();

        if input.starts_with('.') {
            // Execute meta-command by taking the substring after '.'
            MetaCommand::from_str(&input[1..])?.execute();
        } else {
            match Statement::prepare(input) {
                PrepareResult::UnrecognizedStatement => {
                    println!("Unrecognized command: {}", input)
                }
                PrepareResult::SyntaxError(err) => {
                    println!("Syntax error: {}", err)
                }
                PrepareResult::Success(statement) => {
                    let result = vm.execute(statement);
                    println!("{:?}", result);
                }
            }
        }
    }
}
