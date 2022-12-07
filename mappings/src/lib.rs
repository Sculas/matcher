#![feature(option_get_or_insert_default)]

use std::{fs::read_to_string, path::Path};

mod colreader;
mod parser;
pub mod types;
mod utils;

pub mod prelude {
    pub use crate::parser::{EnigmaParser, ParseError, Result as ParseResult};
}

#[derive(Clone, Debug)]
pub struct Mappings(Vec<types::ClassMapping>);

impl Mappings {
    pub fn new(input: String) -> parser::Result<Self> {
        let mut mappings = Vec::new();
        parser::EnigmaParser::new(input, &mut mappings).parse()?;
        Ok(Self(mappings))
    }

    pub fn from_files<P: AsRef<Path>>(files: Vec<P>) -> parser::Result<Self> {
        let mut mappings = Vec::new();
        for file in files {
            parser::EnigmaParser::new(
                read_to_string(file).map_err(parser::ParseError::IoError)?,
                &mut mappings,
            )
            .parse()?;
        }
        Ok(Self(mappings))
    }

    pub fn classes(&self) -> &[types::ClassMapping] {
        &self.0
    }
}
