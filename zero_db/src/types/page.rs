use crate::types::row::Row;
use anyhow::{anyhow, Result};
pub struct Page {
    pub rows: Vec<Row>,
    pub max_rows: usize,
}
impl Page {
    pub fn new(page_size: usize) -> Page {
        Page {
            rows: Vec::new(),
            max_rows: Self::max_rows_per_page(page_size),
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
    pub fn max_rows_per_page(page_size: usize) -> usize {
        page_size / Row::max_size()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Page, Row};
    use rand::distr::Alphanumeric;
    use rand::Rng;
    #[test]
    fn test_new_initialization() {
        let row_size = Row::max_size();
        let page = Page::new(2 * row_size);
        assert_eq!(page.rows.len(), 0);
        assert_eq!(page.max_rows, 2);
    }

    #[test]
    fn test_add_row_happy_path() {
        let row_size = Row::max_size();
        let mut page = Page::new(row_size);

        let row = generate_test_row();
        let result = page.add_row(row);
        assert!(result.is_ok());
        assert_eq!(page.rows.len(), 1);
    }

    #[test]
    fn test_add_row_size_exceeds() {
        let row_size = Row::max_size();
        let mut page = Page::new(row_size);

        let row = generate_test_row();
        let result = page.add_row(row);
        assert!(result.is_ok());
        assert_eq!(page.rows.len(), 1);

        let row = generate_test_row();
        let result = page.add_row(row);
        assert!(!result.is_ok());
        assert_eq!(result.unwrap_err().to_string(), "Page is full");
    }

    #[test]
    fn test_is_full() {
        let mut page = Page::new(Row::max_size());
        assert_eq!(page.is_full(), false);
        let row = generate_test_row();
        let _result = page.add_row(row);
        assert_eq!(page.is_full(), true);
    }

    #[test]
    fn test_max_rows_per_page() {
        let row_size = Row::max_size();
        assert_eq!(Page::max_rows_per_page(0), 0);
        assert_eq!(Page::max_rows_per_page(row_size), 1);
        assert_eq!(Page::max_rows_per_page(2 * row_size), 2);
    }

    fn generate_test_row() -> Row {
        let mut rng = rand::rng();

        Row {
            id: rng.random_range(1..10),
            username: generate_user_name(),
            email: generate_email(),
        }
    }

    fn generate_user_name() -> heapless::String<32> {
        let mut name = heapless::String::<32>::new();
        name.push_str(generate_string(32).as_str())
            .expect("Failed to generate username");
        name
    }

    fn generate_email() -> heapless::String<256> {
        let mut email = heapless::String::<256>::new();
        email
            .push_str(generate_string(256).as_str())
            .expect("Failed to generate email");
        email
    }

    fn generate_string(size: usize) -> String {
        let rng = rand::rng();
        let value: String = rng
            .sample_iter(Alphanumeric)
            .take(size)
            .map(char::from)
            .collect();
        return value;
    }
}
