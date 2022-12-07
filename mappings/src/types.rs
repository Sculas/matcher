#[derive(Clone, Debug)]
pub struct ClassMapping {
    pub obf: String,
    pub deobf: Option<String>,
    pub comment: Option<String>,
    pub methods: Vec<MethodMapping>,
    pub fields: Vec<FieldMapping>,
}

#[derive(Clone, Debug)]
pub struct MethodMapping {
    pub obf: String,
    pub deobf: Option<String>,
    pub comment: Option<String>,
    pub ty: String,
    pub arg_mappings: Vec<MethodArgMapping>,
}

#[derive(Clone, Debug)]
pub struct MethodArgMapping {
    pub index: i64,
    pub deobf: String,
    pub comment: Option<String>,
}

#[derive(Clone, Debug)]
pub struct FieldMapping {
    pub obf: String,
    pub deobf: Option<String>,
    pub comment: Option<String>,
    pub ty: String,
}

#[derive(Clone, Debug)]
pub(crate) struct Descriptor {
    pub obf: String,
    pub deobf: Option<String>,
    pub ty: String,
}
