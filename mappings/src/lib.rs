use std::path::Path;
pub use types::*;

mod colreader;
mod parser;
pub(crate) mod types;
mod utils;

macro_rules! run {
    ($e:block while $cond:expr) => {
        while {
            $e;
            !$cond
        } {}
    };
}
pub(crate) use run;

#[derive(Clone, Debug)]
pub struct Mappings(Vec<ClassMapping>);

impl Mappings {
    pub fn new(input: String) -> Option<Self> {
        None
    }

    pub fn from_files<P: AsRef<Path>>(files: Vec<P>) -> Option<Self> {
        None
    }

    pub fn classes(&self) -> &[ClassMapping] {
        &self.0
    }
}
