use crate::types::{PrepareResult, Row, StatementType};
use anyhow::{anyhow, Result};
use heapless::String as HeapLessString;
pub struct Statement {
    pub(crate) statement_type: StatementType,
    pub(crate) row_to_insert: Option<Row>,
}
impl Statement {
    pub fn prepare(input: &str) -> PrepareResult {
        if input.starts_with("insert") {
            match Statement::parse_insert_input(input) {
                Ok(row) => PrepareResult::Success(Statement {
                    statement_type: StatementType::Insert,
                    row_to_insert: Some(row),
                }),
                Err(err) => PrepareResult::SyntaxError(err.to_string()),
            }
        } else if input.starts_with("select") {
            PrepareResult::Success(Statement {
                statement_type: StatementType::Select,
                row_to_insert: None,
            })
        } else {
            PrepareResult::UnrecognizedStatement
        }
    }

    pub fn parse_insert_input(input: &str) -> Result<Row> {
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the "insert" keyword

        let id: i32 = parts
            .next()
            .ok_or_else(|| anyhow!("No id provided"))?
            .parse()
            .map_err(|_| anyhow!("Id should be a number"))?;

        let username = HeapLessString::<32>::try_from(
            parts
                .next()
                .ok_or_else(|| anyhow!("Username not provided"))?,
        )
            .map_err(|_| anyhow!("Input provided for field(Username) length exceeds the configured length: 32"))?;

        let email = HeapLessString::<256>::try_from(
            parts.next().ok_or_else(|| anyhow!("Email not provided"))?,
        )
            .map_err(|_| anyhow!("Input provided for field(Email) length exceeds the configured length: 256"))?;

        Ok(Row {
            id,
            username,
            email,
        })
    }
}