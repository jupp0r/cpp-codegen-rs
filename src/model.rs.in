use clang::{Entity, EntityKind, EntityVisitResult, TranslationUnit};
use std::vec::Vec;
use std::string::String;
use std::option::Option;
use serde::ser::Serializer;
use rand::{self, Rng};

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
    #[serde(serialize_with="serialize_argument_name_with")]
    pub name: Option<String>,
    pub argument_type: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct TemplateParameter {
    pub type_name: String,
}

impl Model {
    pub fn new(tu: &TranslationUnit) -> Model {
        let mut model = Model { interfaces: vec![] };
        {
            let visitor = |node: Entity, parent: Entity| build_model(&mut model, node, parent);
            tu.get_entity().visit_children(visitor);
        }
        model
    }
}



fn build_model(model: &mut Model, node: Entity, parent: Entity) -> EntityVisitResult {
    if !node.is_in_main_file() {
        return EntityVisitResult::Continue;
    }

    if !is_interface(node) {
        return EntityVisitResult::Recurse;
    }

    match node.get_kind() {
        EntityKind::StructDecl |
        EntityKind::ClassDecl |
        EntityKind::ClassTemplate => {
            model.interfaces.push(Interface {
                name: node.get_name().unwrap(),
                namespaces: parse_namespaces(parent),
                methods: parse_methods(node),
                template_parameters: parse_template_parameters(node),
            });
            EntityVisitResult::Continue
        }
        _ => EntityVisitResult::Recurse,
    }
}

fn parse_namespaces(node: Entity) -> Vec<String> {
    let mut pointer = node;
    let mut namespace_list = vec![];
    while pointer.get_kind() == EntityKind::Namespace {
        namespace_list.push(pointer.get_name().unwrap());
        match pointer.get_lexical_parent() {
            Some(n) => pointer = n,
            None => break,
        }
    }
    namespace_list.reverse();
    namespace_list
}

fn parse_methods(node: Entity) -> Vec<Method> {
    node.get_children()
        .into_iter()
        .filter(|method| {
            method.get_kind() != EntityKind::Destructor &&
            method.get_kind() != EntityKind::Constructor
        })
        .map(parse_method)
        .collect::<Vec<_>>()
}

fn parse_method(node: Entity) -> Method {
    Method {
        name: node.get_name().unwrap(),
        return_type: "void".to_string(),
        arguments: parse_arguments(node),
        template_arguments: parse_template_parameters(node),
    }
}

fn parse_arguments(node: Entity) -> Vec<Argument> {
    node.get_arguments().map_or(vec![],
                                |t| t.into_iter().map(parse_argument).collect::<Vec<_>>())
}

fn parse_argument(t: Entity) -> Argument {
    Argument {
        name: t.get_name(),
        argument_type: t.get_type().unwrap().get_display_name(),
    }
}

fn parse_template_parameters(t: Entity) -> Option<Vec<TemplateParameter>> {
    let params = t.get_children()
        .into_iter()
        .filter_map(|x: Entity| {
            match x.get_kind() {
                EntityKind::TemplateTypeParameter => {
                    Some(TemplateParameter { type_name: x.get_name().unwrap() })
                }
                _ => None,
            }
        })
        .collect::<Vec<_>>();
    if params.is_empty() {
        None
    } else {
        Some(params)
    }
}

fn is_interface(node: Entity) -> bool {
    let res = node.get_children()
        .into_iter()
        .all(|method| method.get_kind() != EntityKind::Method || method.is_virtual_method());
    res
}

fn serialize_argument_name_with<'a, S: Serializer>(name: &'a Option<String>,
                                                   serializer: &mut S)
                                                   -> Result<(), S::Error> {
    let s = match name {
        &Some(ref n) => n.clone(),
        &None => {
            rand::thread_rng()
                .gen_ascii_chars()
                .take(10)
                .collect::<String>().clone()
        },
    };
    serializer.serialize_str(s.to_lowercase().as_str())
}
