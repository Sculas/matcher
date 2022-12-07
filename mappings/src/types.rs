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
    pub args: Vec<String>,
    pub ret: String,
    pub arg_mappings: Vec<ArgMapping>,
}

#[derive(Clone, Debug)]
pub struct ArgMapping {
    pub index: i64,
    pub deobf: String,
    pub comment: Option<String>,
}

#[derive(Clone, Debug)]
pub struct FieldMapping {
    pub obf: String,
    pub deobf: String,
    pub comment: Option<String>,
    pub ty: String,
}
