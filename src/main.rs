#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate serde_json;
extern crate handlebars;
extern crate clang;
extern crate clap;

use clang::{Clang, Index};
use clap::{App, Arg};
use handlebars::Handlebars;

use std::path::Path;
use std::error::Error;
use std::boxed::Box;

mod model;
mod template;

fn main() {
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

    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, false);
    let input = matches.value_of("INPUT").unwrap();
    let tu = index.parser(input).arguments(&[&"-x", &"c++"]).parse().unwrap();
    let model = model::Model::new(&tu);

    let mut handlebars = Handlebars::new();
    let templates = matches.values_of("template").unwrap().collect::<Vec<_>>();
    let template = templates[0];

    handlebars.register_template_file("template", &Path::new(template))
        .ok()
        .unwrap();
    handlebars.register_helper("len",
                               Box::new(template::len));

    let output = handlebars.render("template", &model)
        .unwrap_or_else(|e| e.description().to_owned());
    println!("{}", output);
}
