extern crate clang;

use clang::{Clang, Index};

mod visitor;

fn main() {
    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, false);
    let tu = index.parser("interface.h").arguments(&[&"-x", &"c++"]).parse().unwrap();
    let model = visitor::Model::new(&tu);
    println!("{:?}", model);
}
