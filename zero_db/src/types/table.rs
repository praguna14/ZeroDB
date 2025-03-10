use crate::types::{ExecuteResult, ExecutionFailure, Page, Row, Statement, StatementType};
use anyhow::{anyhow, Context, Result};

pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Page>,
}
impl Table {
    const TABLE_MAX_PAGES: usize = 100;
    const TABLE_PAGE_SIZE: usize = 4096;
    pub fn new() -> Table {
        Table {
            num_rows: 0,
            pages: Vec::new(),
        }
    }
    fn max_rows() -> usize {
        Self::TABLE_MAX_PAGES * Page::max_rows_per_page(Self::TABLE_PAGE_SIZE)
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
            self.pages.push(Page::new(Self::TABLE_PAGE_SIZE));
        }

        if self.pages.last().unwrap().is_full() {
            if self.num_rows < Self::max_rows() {
                return Ok(ExecuteResult::ExecutionFailure(ExecutionFailure::TableFull));
            }
            self.pages.push(Page::new(Self::TABLE_PAGE_SIZE));
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::time::Instant;
    use heapless::String as HeapLessString;

    // Helper function to create a dummy Row.
    fn dummy_row() -> Row {
        Row {
            id: 1,
            username: HeapLessString::<32>::try_from("alice").unwrap(),
            email: HeapLessString::<256>::try_from("alice@example.com").unwrap(),
        }
    }

    // Helper to create a dummy Insert Statement.
    fn insert_statement(row: Option<Row>) -> Statement {
        Statement {
            statement_type: StatementType::Insert,
            row_to_insert: row,
        }
    }

    // Helper to create a dummy Select Statement.
    fn select_statement() -> Statement {
        Statement {
            statement_type: StatementType::Select,
            row_to_insert: None,
        }
    }

    #[test]
    fn test_execute_insert_success() {
        let mut table = Table::new();
        let row = dummy_row();
        let statement = insert_statement(Some(row.clone()));

        let result = table.execute(statement).expect("Insert should succeed");

        // Assuming ExecuteResult::Success carries a vector of inserted rows.
        match result {
            ExecuteResult::Success(rows) => {
                assert_eq!(rows.len(), 1);
                assert_eq!(rows[0], row);
            }
            _ => panic!("Expected ExecuteResult::Success, got {:?}", result),
        }
    }

    #[test]
    fn test_execute_insert_no_row_error() {
        let mut table = Table::new();
        let statement = insert_statement(None);

        let result = table.execute(statement);
        assert!(result.is_err(), "Expected error when no row is provided for insert");
        let error_message = result.unwrap_err().to_string();
        assert!(
            error_message.contains("No row for insertion"),
            "Unexpected error message: {}",
            error_message
        );
    }

    #[test]
    fn test_execute_select_empty() {
        let mut table = Table::new();
        // Initially, the table has no rows.
        let statement = select_statement();
        let result = table.execute(statement).expect("Select should succeed");

        match result {
            ExecuteResult::Success(rows) => {
                assert!(rows.is_empty(), "Expected no rows in a new table");
            }
            _ => panic!("Expected ExecuteResult::Success, got {:?}", result),
        }
    }

    #[test]
    fn test_execute_insert_then_select() {
        let mut table = Table::new();
        let row1 = dummy_row();
        let row2 = Row {
            id: 2,
            username: HeapLessString::<32>::try_from("bob").unwrap(),
            email: HeapLessString::<256>::try_from("bob@example.com").unwrap(),
        };

        // Insert two rows.
        let insert_statement1 = insert_statement(Some(row1.clone()));
        let insert_statement2 = insert_statement(Some(row2.clone()));
        let _result1 = table.execute(insert_statement1).expect("Insert row1 should succeed");
        let _result2 = table.execute(insert_statement2).expect("Insert row2 should succeed");

        // Now select rows from the table.
        let select_statement = select_statement();
        let select_result = table.execute(select_statement).expect("Select should succeed");

        match select_result {
            ExecuteResult::Success(rows) => {
                // Depending on your implementation, the table may contain both rows.
                // Adjust the assertions if your logic updates num_rows or page boundaries differently.
                assert_eq!(rows.len(), 2);
                assert!(rows.contains(&row1));
                assert!(rows.contains(&row2));
            }
            _ => panic!("Expected ExecuteResult::Success, got {:?}", select_result),
        }
    }

    #[test]
    fn test_insert_iterations_avg_time() {
        let mut table = Table::new();
        let row = dummy_row();

        let mut max : u128 = 0;
        for _i in 0..100_000_000 {
            let time_start = Instant::now();
            let statement = insert_statement(Some(row.clone()));

            let _result = table.execute(statement).expect("Insert should succeed");
            let time_end = Instant::now();

            let duration = time_end.duration_since(time_start).as_nanos();

            max = max.max(duration)
        }
        println!("Max execution time: {:?} nanoseconds", max);
        assert!(max <500_000);
    }
}

