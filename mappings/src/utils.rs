use crate::colreader::ColumnReader;
use crate::parser::{ParseError, Result as ParseResult};

pub trait ColumnReaderExt {
    /// Returns the next column, or an error if the column is empty.
    fn next_col_ne(&mut self, token: &'static str) -> ParseResult<String>;
    /// Returns the next column as an [`i64`], or an error if the column is empty or cannot be parsed.
    fn next_col_int(&mut self, token: &'static str) -> ParseResult<i64>;
}

impl ColumnReaderExt for ColumnReader {
    fn next_col_ne(&mut self, token: &'static str) -> ParseResult<String> {
        self.next_col()
            .ok_or_else(|| ParseError::missing_token(self, token))
    }

    fn next_col_int(&mut self, token: &'static str) -> ParseResult<i64> {
        self.next_col_ne(token)?
            .parse()
            .map_err(|e| ParseError::invalid_token(self, token, e))
    }
}
