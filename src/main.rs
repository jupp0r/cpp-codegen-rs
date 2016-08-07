#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
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

use clap::{App, Arg};
use handlebars::Handlebars;

use std::path::Path;
use std::error::Error;
use std::boxed::Box;

pub mod model;
pub mod template;
mod parser_tests;

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("cpp-codegen-rs")
        .version("0.1")
        .author("Jupp Müller <jupp0r@gmail.com>")
        .about("Generate code or documentation from C++ interfaces")
        .arg(Arg::with_name("template")
            .short("t")
            .long("template")
            .multiple(true)
            .required(true)
            .help("Template to render")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("C++ interface to use as model")
            .required(true))
        .get_matches();

    let clang = Clang::new().expect("create clang parser");
    let index = Index::new(&clang, false, false);
    let input = matches.value_of("INPUT").expect("input missing");
    let tu = match index.parser(input).arguments(&[&"-x", &"c++"]).parse() {
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
