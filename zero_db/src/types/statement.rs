use crate::types::{PrepareResult, Row, StatementType};
use anyhow::{anyhow, Result};
use heapless::String as HeapLessString;

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    // A valid input should be parsed successfully.
    #[test]
    fn test_parse_insert_input_valid() {
        let input = "insert 42 john john@example.com";
        let row = Statement::parse_insert_input(input).expect("Parsing should succeed");

        assert_eq!(row.id, 42);
        // Assuming HeapLessString implements PartialEq<&str>
        assert_eq!(row.username, "john");
        assert_eq!(row.email, "john@example.com");
    }

    // When no id is provided, the function should return the correct error.
    #[test]
    fn test_parse_insert_input_no_id() {
        let input = "insert";
        let err = Statement::parse_insert_input(input).unwrap_err();
        assert_eq!(err.to_string(), "No id provided");
    }

    // When the id cannot be parsed as a number.
    #[test]
    fn test_parse_insert_input_invalid_id() {
        let input = "insert abc john john@example.com";
        let err = Statement::parse_insert_input(input).unwrap_err();
        assert_eq!(err.to_string(), "Id should be a number");
    }

    // When the username is missing.
    #[test]
    fn test_parse_insert_input_no_username() {
        let input = "insert 42";
        let err = Statement::parse_insert_input(input).unwrap_err();
        assert_eq!(err.to_string(), "Username not provided");
    }

    // When the email is missing.
    #[test]
    fn test_parse_insert_input_no_email() {
        let input = "insert 42 john";
        let err = Statement::parse_insert_input(input).unwrap_err();
        assert_eq!(err.to_string(), "Email not provided");
    }

    // When the username is too long.
    #[test]
    fn test_parse_insert_input_username_too_long() {
        // Create a username of 33 characters (exceeds capacity of 32)
        let long_username = "a".repeat(33);
        let input = format!("insert 42 {} john@example.com", long_username);
        let err = Statement::parse_insert_input(&input).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Input provided for field(Username) length exceeds the configured length: 32"
        );
    }

    // When the email is too long.
    #[test]
    fn test_parse_insert_input_email_too_long() {
        // Create an email of 257 characters (exceeds capacity of 256)
        let long_email = "a".repeat(257);
        let input = format!("insert 42 john {}", long_email);
        let err = Statement::parse_insert_input(&input).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Input provided for field(Email) length exceeds the configured length: 256"
        );
    }
    #[test]
    fn test_prepare_insert_success() {
        // This input should be parsed successfully.
        let input = "insert 1 alice alice@example.com";
        let result = Statement::prepare(input);

        if let PrepareResult::Success(statement) = result {
            assert_eq!(statement.statement_type, StatementType::Insert);
            assert!(statement.row_to_insert.is_some());

            let row = statement.row_to_insert.unwrap();
            // Verify that the row fields are as expected.
            assert_eq!(row.id, 1);
            // Assuming HeapLessString implements PartialEq<&str> or a similar comparison.
            assert_eq!(row.username, "alice");
            assert_eq!(row.email, "alice@example.com");
        }
    }

    #[test]
    fn test_prepare_insert_syntax_error() {
        // Missing username and email should trigger a syntax error.
        let input = "insert 1";
        let result = Statement::prepare(input);
        match result {
            PrepareResult::SyntaxError(err) => {
                // The error string may mention the missing field.
                assert!(
                    err.contains("Username not provided") || err.contains("Email not provided"),
                    "Unexpected error message: {}",
                    err
                );
            }
            _ => panic!("Expected SyntaxError for incomplete insert command, got {:?}", result),
        }
    }

    #[test]
    fn test_prepare_select_success() {
        let input = "select";
        let result = Statement::prepare(input);
        match result {
            PrepareResult::Success(statement) => {
                // For a select statement, we expect StatementType::Select and no row.
                assert_eq!(statement.statement_type, StatementType::Select);
                assert!(statement.row_to_insert.is_none());
            }
            _ => panic!("Expected Success for a select command, got {:?}", result),
        }
    }

    #[test]
    fn test_prepare_unrecognized_statement() {
        let input = "foobar";
        let result = Statement::prepare(input);
        match result {
            PrepareResult::UnrecognizedStatement => {
                // Test passes as we expect unrecognized statement.
            }
            _ => panic!("Expected UnrecognizedStatement for unrecognized input, got {:?}", result),
        }
    }
}
