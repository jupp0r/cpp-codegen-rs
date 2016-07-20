#[cfg(test)]
mod tests {
    use model::Model;

    use std::fs::File;
    use std::io::Write;
    use clang::{Clang, Index};
    use tempdir::TempDir;


    const INTERFACE: &'static str = r#"
#pragma once

namespace rtc { namespace sample {

struct Interface {
    virtual ~Interface() = default;
    virtual void method(int foo) = 0;
    virtual int foo(double bar) = 0;
    virtual bool baz() {return true};
};

}}
"#;

    fn create_model() -> Model {
        let tmp_dir = TempDir::new("cpp-codegen").expect("create temp dir");
        let file_path = tmp_dir.path().join("interface.h");
        let mut tmp_file = File::create(&file_path).expect("create temp file");
        tmp_file.write(INTERFACE.as_bytes()).expect("write file");

        let clang = Clang::new().expect("instantiate clang");
        let index = Index::new(&clang, false, false);
        let tu = index.parser(&file_path).arguments(&[&"-x", &"c++"]).parse().expect("parse interface");
        Model::new(&tu)
    }

    #[test]
    fn should_parse() {
        create_model();
    }
}
