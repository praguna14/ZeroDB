use crate::types::row::Row;
use crate::types::statement::Statement;
use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub enum MetaCommand {
    Exit,
}
impl MetaCommand {
    pub fn from_str(input: &str) -> Result<MetaCommand> {
        match input {
            "exit" => Ok(MetaCommand::Exit),
            _ => Err(anyhow!("Invalid meta-command")),
        }
    }
    pub fn execute(self) {
        match self {
            MetaCommand::Exit => std::process::exit(0),
        }
    }
}

#[derive(Debug)]
pub enum PrepareResult {
    Success(Statement),
    SyntaxError(String),
    UnrecognizedStatement,
}

#[derive(Debug)]
pub enum ExecutionFailure {
    TableFull,
}

#[derive(Debug)]
pub enum ExecuteResult {
    Success(Vec<Row>),
    ExecutionFailure(ExecutionFailure),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum StatementType {
    Insert,
    Select,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_meta_command_from_str_exit() {
        assert_eq!(MetaCommand::Exit, MetaCommand::from_str("exit").unwrap());
    }
    
    #[test]
    pub fn test_meta_command_from_str_invalid() {
        let result: Result<MetaCommand> = MetaCommand::from_str("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid meta-command");
    }
}
