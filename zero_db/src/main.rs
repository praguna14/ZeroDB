use anyhow::{anyhow, Result};
use std::io::{self, Write};

enum MetaCommand {
    Exit,
}

impl MetaCommand {
    fn from_str(input: &str) -> Result<MetaCommand> {
        match input {
            "exit" => Ok(MetaCommand::Exit),
            _ => Err(anyhow!("Invalid metacommand")),
        }
    }

    fn execute(self) {
        match self {
            MetaCommand::Exit => std::process::exit(0),
        }
    }
}

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

enum PrepareResult {
    Success(Statement),
    UnrecognizedStatement,
}

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType,
}
impl Statement {
    fn prepare(input: &str) -> PrepareResult {
        if input.starts_with("insert") {
            return PrepareResult::Success(Statement {
                statement_type: StatementType::Insert,
            });
        }

        if input.starts_with("select") {
            return PrepareResult::Success(Statement {
                statement_type: StatementType::Select,
            });
        }

        PrepareResult::UnrecognizedStatement
    }

    fn execute(self) -> Result<()> {
        match self.statement_type {
            StatementType::Insert => {
                println!("This is where we would do an insert");
            }
            StatementType::Select => {
                println!("This is where we would do a select");
            }
        }

        Ok(())
    }
}

#[allow(unreachable_code)]
fn main() -> Result<()> {
    loop {
        let mut input = String::new();
        print!("db > ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        let input = input.trim();

        if input.starts_with('.') {
            // Execute metacommand by taking the substring after '.'
            MetaCommand::from_str(&input[1..])?.execute();
        } else {
            let prepare_res = Statement::prepare(input);
            match prepare_res {
                PrepareResult::UnrecognizedStatement => println!("Unrecognized command: {}", input),
                PrepareResult::Success(statement) => statement.execute()?,
            }
        }
    }

    Ok(())
}
