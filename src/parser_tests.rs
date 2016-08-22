#[cfg(test)]
mod tests {
    use model::{Model, TemplateParameter};

    use std::fs::File;
    use std::io::Write;
    use clang::{Clang, Index};
    use tempdir::TempDir;

    use serde_json;
    use std::fmt::Debug;

    use response_file::ExtendWithResponseFile;

    const INTERFACE: &'static str = r#"
namespace foo { namespace sample {

struct Interface {
    virtual ~Interface() = default;
    virtual void method(int foo) = 0;
    virtual int foo(double bar) = 0;
    virtual bool baz() {return true};
};

}}
"#;

    const TEMPLATE_INTERFACE: &'static str = r#"
namespace foo { namespace bar {

template <typename T, typename U, class V>
class Baz {
    virtual ~Baz() = default;
    Baz(const Baz&) = delete;
    virtual bool boo() = 0;
};

}}
"#;

    fn create_model(input: &str) -> Model {
        create_model_with_args(input, &[&"-x", &"c++", &"-std=c++14"])
    }

    fn create_model_with_args<S: AsRef<str> + Debug>(input: &str, args: &[S]) -> Model {
        println!("input: {:?} args: {:?}", input, args);
        let tmp_dir = TempDir::new("cpp-codegen").expect("create temp dir");
        let file_path = tmp_dir.path().join("interface.h");
        let mut tmp_file = File::create(&file_path).expect("create temp file");
        tmp_file.write(input.as_bytes()).expect("write file");
        let clang = Clang::new().expect("instantiate clang");
        let index = Index::new(&clang, false, false);
        let tu = index.parser(&file_path)
            .arguments(args)
            .parse()
            .expect("parse interface");
        Model::new(&tu)
    }


    #[test]
    fn should_parse() {
        create_model(INTERFACE);
    }

    #[test]
    fn should_list_namespaces() {
        let model = create_model(INTERFACE);
        assert!(model.interfaces[0].namespaces == vec!["foo".to_string(), "sample".to_string()]);
    }

    #[test]
    fn should_not_include_destructors() {
        let model = create_model(INTERFACE);
        assert!(!model.interfaces[0]
            .clone()
            .methods
            .into_iter()
            .any(|x| x.name == "~Interface"));
    }

    #[test]
    fn should_parse_template_class() {
        create_model(TEMPLATE_INTERFACE);
    }

    #[test]
    fn should_list_template_parameters() {
        let model = create_model(TEMPLATE_INTERFACE);
        assert!(model.interfaces.len() > 0);
        assert_eq!(model.interfaces[0]
                       .clone()
                       .template_parameters
                       .unwrap(),
                   vec![TemplateParameter { type_name: "T".to_string() },
                        TemplateParameter { type_name: "U".to_string() },
                        TemplateParameter { type_name: "V".to_string() }]);
    }

    #[test]
    fn should_generate_random_names_for_anonymous_method_arguments() {

        const INTERFACE_WITH_NAMELESS_ARGUMENT_METHODS: &'static str = r#"
            struct Foo {
                virtual void noArgumentName(double) = 0;
            };
            "#;

        let model = create_model(INTERFACE_WITH_NAMELESS_ARGUMENT_METHODS);
        assert!(serde_json::ser::to_string(&model.interfaces[0].methods[0].arguments[0].name)
            .unwrap()
            .len() > 0)
    }

    #[test]
    fn should_parse_template_methods() {

        const INTERFACE_WITH_TEMPLATE_METHODS: &'static str = r#"
            struct Foo {
                template <typename T> void foo(T bar);
            };
            "#;

        let model = create_model(INTERFACE_WITH_TEMPLATE_METHODS);
        assert_eq!(model.interfaces[0]
                           .clone()
                           .methods[0]
                       .clone()
                       .template_arguments
                       .unwrap(),
                   vec![TemplateParameter { type_name: "T".to_string() }])
    }

    #[test]
    fn should_parse_arguments_with_namespaced_types() {
        const INTERFACE_WITH_NAMESPACED_METHOD_PARAMETERS: &'static str = r#"
           namespace a { namespace b {
             using c = int;
           }}

           struct A {
               virtual void method(a::b::c abc);
           };
           "#;

        let model = create_model(INTERFACE_WITH_NAMESPACED_METHOD_PARAMETERS);
        println!("{:?}", model);
        assert_eq!(model.interfaces[0]
                               .clone()
                               .methods[0]
                           .clone()
                           .arguments[0]
                       .argument_type,
                   "a::b::c".to_string());
        assert_eq!(model.interfaces[0]
                               .clone()
                               .methods[0]
                           .clone()
                           .arguments[0]
                       .name,
                   Some("abc".to_string()));
    }

    #[test]
    fn should_support_response_file() {
        const INTERFACE_WITH_RESPONSE_FILE_DEFINE: &'static str = r#"
           #ifdef FLAG
           struct A {
               virtual void method(int abc);
           };
           #endif
       "#;
        const RESPONSE_FILE: &'static str = "-x;c++;-std=c++14;-DFLAG";
        let tmp_dir = TempDir::new("cpp-codegen").expect("create temp dir");
        let response_file_path = tmp_dir.path().join("interface.rsp");
        let mut response_file = File::create(&response_file_path).expect("create temp file");
        response_file.write(RESPONSE_FILE.as_bytes()).expect("write file");
        let model = create_model_with_args(INTERFACE_WITH_RESPONSE_FILE_DEFINE,
                                           &vec!["@".to_string() +
                                            &response_file_path.to_string_lossy()]
                                           .into_iter()
                                           .extend_with_response_file()
                                           .collect::<Vec<_>>());

        assert_eq!(model.interfaces[0].name, "A".to_string());
    }
}
