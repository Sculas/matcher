use crate::colreader::ColumnReader;
use crate::parser::{ParseError, Result as ParseResult};

pub trait ColumnReaderExt {
    /// Returns the next column, or an error if the column is empty.
    fn next_col_ne(&mut self, token: &'static str) -> ParseResult<&str>;
}

impl<'a> ColumnReaderExt for ColumnReader<'a> {
    fn next_col_ne(&mut self, token: &'static str) -> ParseResult<&'a str> {
        self.next_col().ok_or_else(|| ParseError::missing_token(self, token))
    }
}
