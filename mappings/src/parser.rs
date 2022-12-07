use std::fmt::Display;
use std::num::ParseIntError;

use crate::utils::ColumnReaderExt;

use super::colreader::ColumnReader;
use super::types::*;

#[derive(Debug)]
pub enum ParseError {
    MissingToken {
        token: &'static str,
        eof: bool,
        eol: bool,
        line: usize,
        col: usize,
    },
    InvalidToken {
        token: &'static str,
        line: usize,
        col: usize,
        from: ParseIntError,
    },
    IoError(std::io::Error),
}

impl ParseError {
    pub fn missing_token(parser: &ColumnReader, token: &'static str) -> Self {
        Self::MissingToken {
            token,
            eof: parser.eof(),
            eol: parser.eol(),
            line: parser.line,
            col: parser.pos + 1,
        }
    }

    pub fn invalid_token(parser: &ColumnReader, token: &'static str, from: ParseIntError) -> Self {
        Self::InvalidToken {
            token,
            line: parser.line,
            col: parser.pos + 1,
            from,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingToken {
                token,
                eof,
                eol,
                line,
                col,
            } => {
                write!(
                    f,
                    "unexpected {} at {}:{}, expected {}",
                    match () {
                        _ if *eof => "EOF",
                        _ if *eol => "EOL",
                        _ => "token",
                    },
                    line,
                    col,
                    token
                )
            }
            Self::InvalidToken {
                token,
                line,
                col,
                from,
            } => {
                write!(
                    f,
                    "invalid token at {}:{}, expected {}. error: {}",
                    line, col, token, from
                )
            }
            Self::IoError(e) => write!(f, "io error: {}", e),
        }
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

pub struct EnigmaParser<'a>(ColumnReader, &'a mut Vec<ClassMapping>);

impl<'a> EnigmaParser<'a> {
    pub fn new(src: String, classes: &'a mut Vec<ClassMapping>) -> Self {
        Self(ColumnReader::new(src), classes)
    }

    pub fn parse(&mut self) -> Result<()> {
        loop {
            if self.0.next_col_expect("CLASS") {
                // CLASS <name-a> [<name-b>]
                let class = self.parse_class(0, None, None)?;
                self.1.push(class);
            }
            if !self.0.next_line(0) {
                break;
            }
        }
        Ok(())
    }

    fn parse_class(
        &mut self,
        indent: usize,
        outer_a: Option<&String>,
        outer_b: Option<&String>,
    ) -> Result<ClassMapping> {
        let original_a = self.0.next_col_ne("class-name-a")?;
        let mut a = original_a.clone();
        if let Some(outer_a) = outer_a {
            a = format!("{}${}", outer_a, a);
        }
        let mut b = self.0.next_col();
        if let Some(outer_b) = outer_b {
            b = Some(format!("{}${}", outer_b, b.unwrap_or(original_a)));
        }

        let mut comment = None;
        let mut methods = Vec::new();
        let mut fields = Vec::new();
        while self.0.next_line(indent + 1) {
            match self
                .0
                .next_col_ne("nested class, method, field or comment")?
                .as_str()
            {
                "CLASS" => {
                    // CLASS <name-a> [<name-b>]
                    let class = self.parse_class(indent + 1, Some(&a), b.as_ref())?;
                    self.1.push(class);
                }
                "METHOD" => {
                    // METHOD <name-a> [<name-b>] <desc-a>
                    let descriptor = self.parse_descriptor()?;
                    let method = self.parse_method_body(indent + 1, descriptor)?;
                    methods.push(method);
                }
                "FIELD" => {
                    // FIELD <name-a> [<name-b>] <desc-a>
                    let descriptor = self.parse_descriptor()?;
                    let field = self.parse_field_body(indent + 1, descriptor)?;
                    fields.push(field);
                }
                "COMMENT" => {
                    // COMMENT <comment>
                    let comment: &mut String = comment.get_or_insert_default();
                    comment.push_str(&self.parse_comment());
                }
                _ => {
                    return Err(ParseError::missing_token(
                        &self.0,
                        "nested class, method, field or comment",
                    ))
                } // FIXME: don't duplicate the error message
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

    fn parse_method_body(
        &mut self,
        indent: usize,
        descriptor: Descriptor,
    ) -> Result<MethodMapping> {
        let mut comment = None;
        let mut arg_mappings = Vec::new();
        while self.0.next_line(indent + 1) {
            match self.0.next_col_ne("method comment or arg")?.as_str() {
                "COMMENT" => {
                    // COMMENT <comment>
                    let comment: &mut String = comment.get_or_insert_default();
                    comment.push_str(&self.parse_comment());
                }
                "ARG" => {
                    // ARG <index> <name-b>
                    let arg = self.parse_method_arg(indent + 1)?;
                    arg_mappings.push(arg);
                }
                _ => return Err(ParseError::missing_token(&self.0, "method comment or arg")), // FIXME: don't duplicate the error message
            }
        }

        Ok(MethodMapping {
            obf: descriptor.obf,
            deobf: descriptor.deobf,
            comment,
            ty: descriptor.ty,
            arg_mappings,
        })
    }

    fn parse_method_arg(&mut self, indent: usize) -> Result<MethodArgMapping> {
        let index = self.0.next_col_int("arg index")?;
        let name = self.0.next_col_ne("arg name")?;
        let mut comment = None;
        while self.0.next_line(indent + 1) {
            match self.0.next_col_ne("arg comment")?.as_str() {
                "COMMENT" => {
                    // COMMENT <comment>
                    let comment: &mut String = comment.get_or_insert_default();
                    comment.push_str(&self.parse_comment());
                }
                _ => return Err(ParseError::missing_token(&self.0, "arg comment")), // FIXME: don't duplicate the error message
            }
        }
        Ok(MethodArgMapping {
            index,
            deobf: name,
            comment,
        })
    }

    fn parse_field_body(&mut self, indent: usize, descriptor: Descriptor) -> Result<FieldMapping> {
        let mut comment = None;
        while self.0.next_line(indent + 1) {
            match self.0.next_col_ne("field comment")?.as_str() {
                "COMMENT" => {
                    // COMMENT <comment>
                    let comment: &mut String = comment.get_or_insert_default();
                    comment.push_str(&self.parse_comment());
                }
                _ => return Err(ParseError::missing_token(&self.0, "field comment")), // FIXME: don't duplicate the error message
            }
        }
        Ok(FieldMapping {
            obf: descriptor.obf,
            deobf: descriptor.deobf,
            comment,
            ty: descriptor.ty,
        })
    }

    fn parse_comment(&mut self) -> String {
        self.0.peek_next_cols()
    }

    fn parse_descriptor(&mut self) -> Result<Descriptor> {
        // <name-a> [<name-b>] <desc-a>
        let name_a = self.0.next_col_ne("name-a")?;
        let name_or_desc = self.0.next_col_ne("desc")?;
        let desc = self.0.next_col();
        Ok(match desc {
            Some(desc) => Descriptor {
                obf: name_a,
                deobf: Some(name_or_desc),
                ty: desc,
            },
            None => Descriptor {
                obf: name_a,
                deobf: None,
                ty: name_or_desc,
            },
        })
    }
}
