use crate::parser::{EnigmaParser, Result, Rule};
use pest_consume::Parser;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Mappings(pub(super) Vec<ClassMapping>);

impl Mappings {
    pub fn new(input: String) -> Result<Self> {
        let nodes = EnigmaParser::parse(Rule::mappings, &input)?;
        EnigmaParser::mappings(nodes.single()?)
    }

    pub fn from_files<P: AsRef<Path>>(files: Vec<P>) -> Result<Self> {
        let mut mappings = Mappings(Vec::new());
        for file in files {
            let input = std::fs::read_to_string(file).expect("error reading file");
            let nodes = EnigmaParser::parse(Rule::mappings, &input)?;
            EnigmaParser::mappings_into(nodes.single()?, &mut mappings)?;
        }
        Ok(mappings)
    }

    pub fn classes(&self) -> &[ClassMapping] {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct ClassMapping {
    pub obf: String,
    pub deobf: Option<String>,
    pub methods: Vec<MethodMapping>,
    pub fields: Vec<FieldMapping>,
}

#[derive(Clone, Debug)]
pub struct MethodMapping {
    pub obf: String,
    pub deobf: Option<String>,
    pub args: Vec<String>,
    pub ret: String,
    pub arg_mappings: Vec<ArgMapping>,
}

#[derive(Clone, Debug)]
pub struct ArgMapping {
    pub index: i64,
    pub deobf: String,
}

#[derive(Clone, Debug)]
pub struct FieldMapping {
    pub obf: String,
    pub deobf: String,
    pub ty: String,
}

#[derive(Clone, Debug)]
pub(crate) enum FOM {
    Method(MethodMapping),
    Field(FieldMapping),
}

impl FOM {
    pub(crate) fn method(self) -> Option<MethodMapping> {
        match self {
            FOM::Method(m) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn field(self) -> Option<FieldMapping> {
        match self {
            FOM::Field(f) => Some(f),
            _ => None,
        }
    }
}
