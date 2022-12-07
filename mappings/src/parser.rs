use std::fmt::Display;

use crate::utils::ColumnReaderExt;

use super::colreader::ColumnReader;
use super::run;
use super::types::*;

#[derive(Debug)]
pub enum ParseError {
    MissingToken {
        token: &'static str,
        eof: bool,
        line: usize,
        col: usize,
    },
}

impl ParseError {
    pub fn missing_token(parser: &ColumnReader, token: &'static str) -> Self {
        Self::MissingToken {
            token,
            eof: parser.eof(),
            line: parser.line,
            col: parser.pos,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingToken {
                token,
                eof,
                line,
                col,
            } => {
                write!(
                    f,
                    "unexpected {} at {}:{}, expected {}",
                    if *eof { "EOF" } else { "token" },
                    line,
                    col,
                    token
                )
            }
        }
    }
}

pub(crate) type Result<T> = std::result::Result<T, ParseError>;

struct EnigmaParser<'a>(ColumnReader<'a>, &'a mut Vec<ClassMapping>);

impl<'a> EnigmaParser<'a> {
    pub fn new(src: &'a str, classes: &'a mut Vec<ClassMapping>) -> Self {
        Self(ColumnReader::new(src), classes)
    }

    pub fn parse(&mut self) -> Result<()> {
        run!({
            if self.0.next_col_expect("CLASS") { // CLASS <name-a> [<name-b>]
                let class = self.parse_class(0, None, None)?;
                self.1.push(class);
            }
        } while self.0.next_line(0));
        Ok(())
    }

    fn parse_class(
        &mut self,
        indent: usize,
        outer_a: Option<&str>,
        outer_b: Option<&str>,
    ) -> Result<ClassMapping> {
        let original_a = self.0.next_col_ne("class-name-a")?;
        let mut a = original_a.clone();
        if let Some(outer_a) = outer_a {
            a = &format!("{}${}", outer_a, a);
        }
        let mut b = self.0.next_col();
        if let Some(outer_b) = outer_b {
            b = Some(&format!("{}${}", outer_b, b.unwrap_or(original_a)));
        }

        let mut comment = None;
        let mut methods = Vec::new();
        let mut fields = Vec::new();
        while self.0.next_line(indent) {
            match self.0.next_col_ne("type")? {
                "CLASS" => {
                    let class = self.parse_class(indent + 1, Some(a), b)?;
                    self.1.push(class);
                }
                // "METHOD" => {
                //     let method = self.parse_method(indent + 1, a, b)?;
                //     methods.push(method);
                // }
                // "FIELD" => {
                //     let field = self.parse_field(indent + 1, a, b)?;
                //     fields.push(field);
                // }
                "COMMENT" => {
                    comment = Some(self.0.next_col_ne("comment")?.into());
                }
                _ => return Err(ParseError::missing_token(&self.0, "type")),
            }
        }

        Ok(ClassMapping {
            obf: a.into(),
            deobf: b.map(|s| s.into()),
            comment,
            methods,
            fields,
        })
    }
}
