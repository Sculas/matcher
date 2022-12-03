use crate::parser::Node;
use std::num::ParseIntError;

pub trait NodeExt {
    fn str(&self) -> String;
    fn int(&self) -> Result<i64, ParseIntError>;
}

impl NodeExt for Node<'_> {
    fn str(&self) -> String {
        self.as_str().into()
    }

    fn int(&self) -> Result<i64, ParseIntError> {
        self.as_str().parse()
    }
}

pub struct MethodDescriptor {
    pub name: Option<String>,
    pub args: Vec<String>,
    pub ty: String,
}
