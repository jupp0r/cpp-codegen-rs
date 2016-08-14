use clap::{App, AppSettings, Arg};

pub fn build_argument_parser<'a, 'b>() -> App<'a, 'b> {
    App::new("cpp-codegen-rs")
        .version("0.1")
        .author("Jupp Müller <jupp0r@gmail.com>")
        .about("Generate code or documentation from C++ interfaces")
        .setting(AppSettings::AllowLeadingHyphen)
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::DontDelimitTrailingValues)
        .arg(Arg::with_name("template")
            .short("t")
            .long("template")
            .required(true)
            .help("Template to render")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("C++ interface to use as model")
            .required(true))
        .arg(Arg::with_name("FLAGS")
            .help("Compiler flags needed to parse the input file")
            .required(false)
            .multiple(true))
}

#[test]
fn should_recognize_compiler_flags() {
    let m = build_argument_parser()
        .get_matches_from(vec!["", "interface.h", "-t", "foobar", "--", "-DFOO", "-DBAR"]);
    println!("{:?}", m);
    assert_eq!(m.values_of("FLAGS").unwrap().collect::<Vec<_>>(),
               ["-DFOO", "-DBAR"]);
}
