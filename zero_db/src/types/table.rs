use crate::types::{ExecuteResult, ExecutionFailure, Page, Row, Statement, StatementType};
use anyhow::{anyhow, Context, Result};

pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Page>,
}
impl Table {
    const TABLE_MAX_PAGES: usize = 100;
    pub fn new() -> Table {
        Table {
            num_rows: 0,
            pages: Vec::new(),
        }
    }
    fn max_rows() -> usize {
        Self::TABLE_MAX_PAGES * Page::max_rows_per_page()
    }

    pub fn execute(&mut self, statement: Statement) -> Result<ExecuteResult> {
        match statement.statement_type {
            StatementType::Insert => Self::execute_insert(self, statement.row_to_insert),
            StatementType::Select => Self::execute_select(self),
        }
    }

    fn execute_insert(&mut self, row: Option<Row>) -> Result<ExecuteResult> {
        let row = row.ok_or_else(|| anyhow!("No row for insertion"))?;

        if self.pages.is_empty() {
            self.pages.push(Page::new());
        }

        if self.pages.last().unwrap().is_full() {
            if self.num_rows < Self::max_rows() {
                return Ok(ExecuteResult::ExecutionFailure(ExecutionFailure::TableFull));
            }
            self.pages.push(Page::new());
        }

        self.pages
            .last_mut()
            .context("Last page not found")?
            .add_row(row.clone())?;

        let rows_inserted = vec![row.clone()];
        Ok(ExecuteResult::Success(rows_inserted))
    }

    fn execute_select(&mut self) -> Result<ExecuteResult> {
        let rows: Vec<Row> = self.pages.iter().flat_map(|page| page.rows.clone()).collect();
        Ok(ExecuteResult::Success(rows))
    }
}
