use std::vec::Vec;
use std::string::String;
use std::option::Option;
use parser;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Model {
    pub interfaces: Vec<Interface>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Interface {
    pub name: String,
    pub namespaces: Vec<String>,
    pub methods: Vec<Method>,
    pub template_parameters: Option<Vec<TemplateParameter>>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Method {
    pub name: String,
    pub return_type: String,
    pub arguments: Vec<Argument>,
    pub template_arguments: Option<Vec<TemplateParameter>>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Argument {
    #[serde(serialize_with="parser::serialize_argument_name_with")]
    pub name: Option<String>,
    pub argument_type: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct TemplateParameter {
    pub type_name: String,
}
