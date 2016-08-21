use clap::{App, Arg};

pub fn build_argument_parser<'a, 'b>() -> App<'a, 'b> {
    App::new("cpp-codegen-rs")
        .version("0.1")
        .author("Jupp Müller <jupp0r@gmail.com>")
        .about("Generate code or documentation from C++ interfaces")
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Verbose output to help template authoring"))
        .arg(Arg::with_name("template")
            .short("t")
            .long("template")
            .required(true)
            .help("Template to render")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("INPUT")
            .help("C++ interface to use as model")
            .required(true))
        .arg(Arg::with_name("FLAGS")
            .help("Compiler flags needed to parse the input file")
            .required(false)
            .multiple(true))
}

#[test]
fn should_recognize_verbosity_flags() {
    let m = build_argument_parser()
        .get_matches_from(vec!["", "interface.h", "-vv", "-t", "foobar"]);
    println!("{:?}", m);
    assert_eq!(m.occurrences_of("verbose"), 2)
}

#[test]
fn should_parse_template_arguments() {
    let m = build_argument_parser().get_matches_from(vec!["",
                                                          "interface.h",
                                                          "-vvv",
                                                          "-t",
                                                          "foo",
                                                          "-t",
                                                          "bar",
                                                          "--",
                                                          "-DFOO",
                                                          "-DBAR"]);
    assert_eq!(m.values_of("template").unwrap().collect::<Vec<_>>(),
               vec!["foo", "bar"]);
}

#[test]
fn should_recognize_compiler_flags() {
    let m = build_argument_parser()
        .get_matches_from(vec!["", "interface.h", "-t", "foobar", "--", "-DFOO", "-DBAR"]);
    println!("{:?}", m);
    assert_eq!(m.values_of("FLAGS").unwrap().collect::<Vec<_>>(),
               ["-DFOO", "-DBAR"]);
}
