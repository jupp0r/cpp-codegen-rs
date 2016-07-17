use clang::{Entity, EntityKind, EntityVisitResult, TranslationUnit};
use std::vec::Vec;
use std::string::String;
use std::option::Option;

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    interfaces: Vec<Interface>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interface {
    name: String,
    namespaces: Vec<String>,
    methods: Vec<Method>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Method {
    name: String,
    return_type: String,
    arguments: Vec<Argument>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Argument {
    name: Option<String>,
    argument_type: String,
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

    if node.get_kind() == EntityKind::StructDecl {
        model.interfaces.push(Interface {
            name: node.get_name().unwrap_or("unknown".to_string()),
            namespaces: parse_namespaces(parent),
            methods: parse_pure_virtual_methods(node),
        });
        EntityVisitResult::Continue
    } else {
        EntityVisitResult::Recurse
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

fn parse_pure_virtual_methods(node: Entity) -> Vec<Method> {
    node.get_children()
        .into_iter()
        .filter(|method| method.is_pure_virtual_method())
        .map(parse_method)
        .collect::<Vec<_>>()
}

fn parse_method(node: Entity) -> Method {
    Method {
        name: node.get_name().unwrap(),
        return_type: "void".to_string(),
        arguments: parse_arguments(node),
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

fn is_interface(node: Entity) -> bool {
    let res = node.get_children().into_iter().all(|method| {
        method.get_kind() == EntityKind::Destructor || method.is_pure_virtual_method()
    });
    res
}
