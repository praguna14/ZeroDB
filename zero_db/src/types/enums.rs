use anyhow::{anyhow, Result};
use crate::types::row::Row;
use crate::types::statement::Statement;

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

pub enum StatementType {
    Insert,
    Select,
}