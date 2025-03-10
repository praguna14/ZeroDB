use crate::types::{ExecuteResult, Statement, Table};
use anyhow::Result;

pub struct VirtualMachine {
    table: Table,
}
impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            table: Table::new(),
        }
    }

    pub fn execute(&mut self, statement: Statement) -> Result<ExecuteResult> {
        Ok(self.table.execute(statement)?)
    }
}
