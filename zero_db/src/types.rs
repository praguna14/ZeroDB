use anyhow::{anyhow, Context, Result};
use heapless::String as HeapLessString;

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
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub id: i32,
    pub username: HeapLessString<32>,
    pub email: HeapLessString<256>,
}
impl Row {
    pub fn max_size() -> usize {
        size_of::<i32>() + size_of::<HeapLessString<32>>() + size_of::<HeapLessString<256>>()
    }
}

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
#[derive(Debug)]
pub enum ExecutionFailure {
    TableFull,
}
#[derive(Debug)]
pub enum ExecuteResult {
    Success(Vec<Row>),
    ExecutionFailure(ExecutionFailure),
}
pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Page>,
}
impl Table {
    const TABLE_MAX_PAGES: usize = 100;
    fn new() -> Table {
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

pub enum StatementType {
    Insert,
    Select,
}

pub struct Statement {
    statement_type: StatementType,
    row_to_insert: Option<Row>,
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
        .map_err(|_| anyhow!("Input provided for field(Username) length exceeds the configured length"))?;
        
        let email = HeapLessString::<256>::try_from(
            parts.next().ok_or_else(|| anyhow!("Email not provided"))?,
        )
        .map_err(|_| anyhow!("Input provided for field(Email) length exceeds the configured length"))?;

        Ok(Row {
            id,
            username,
            email,
        })
    }
}

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
