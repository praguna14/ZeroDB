use crate::types::row::Row;
use anyhow::{anyhow, Result};
pub struct Page {
    pub rows: Vec<Row>,
    pub max_rows: usize,
}
impl Page {
    const PAGE_SIZE: usize = 4096;
    pub fn new() -> Page {
        Page {
            rows: Vec::new(),
            max_rows: Self::max_rows_per_page(),
        }
    }
    pub fn add_row(&mut self, row: Row) -> Result<()> {
        if self.is_full() {
            return Err(anyhow!("Page is full"));
        }

        self.rows.push(row);
        Ok(())
    }
    pub fn is_full(&self) -> bool {
        self.rows.len() == self.max_rows
    }
    pub fn max_rows_per_page() -> usize {
        Self::PAGE_SIZE / Row::max_size()
    }
}
