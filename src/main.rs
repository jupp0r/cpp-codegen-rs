extern crate serde_json;
extern crate serde;
extern crate handlebars;
extern crate clang;
extern crate clap;
extern crate rand;

#[cfg(test)]
extern crate tempdir;

#[macro_use]
extern crate log;
extern crate env_logger;

use clang::{Clang, Index};

use handlebars::Handlebars;

use std::path::Path;
use std::error::Error;
use std::boxed::Box;

mod model;
mod template;
mod cmdline;
mod parser;

#[cfg(test)]
mod parser_tests;

fn main() {
    env_logger::init().unwrap();

    let matches = cmdline::build_argument_parser().get_matches();

    let clang = Clang::new().expect("create clang parser");
    let index = Index::new(&clang, false, false);
    let input = matches.value_of("INPUT").expect("input missing");
    let clang_flags =
        match matches.values_of("Flags") {
            None => vec!["-x", "c++"],
            Some(vals) => vec!["-x", "c++"].into_iter().chain(vals).collect::<Vec<_>>(),
        };
    let tu = match index.parser(input).arguments(&clang_flags).parse() {
        Ok(x) => x,
        Err(e) => panic!(format!("{:?}", e)),
    };
    let model = model::Model::new(&tu);

    let template_file_name = matches.values_of("template").unwrap().nth(0).unwrap();

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("len", Box::new(template::len));

    match handlebars.register_template_file("template", &Path::new(template_file_name)) {
        Err(e) => panic!(format!("{:?}", e)),
        _ => (),
    };

    let output = handlebars.render("template", &model)
        .unwrap_or_else(|e| e.description().to_owned());
    println!("{}", output);
}
