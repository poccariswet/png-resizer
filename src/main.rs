extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("png-resizer")
        .version("1.0")
        .author("poccariswet <poccariswet@gmail.com>")
        .about("png resize command line interface")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("NUM")
                .help("Sets a width value")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("hegiht")
                .value_name("NUM")
                .help("Sets a height value")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .value_name("FILES")
                .help("Sets for resize files")
                .index(1)
                .multiple(true)
                .required(true),
        )
        .get_matches();

    if let Some(in_files) = matches.values_of("input") {
        for file in in_files {
            println!("An input file: {}", file);
        }
    }

    if let Some(c) = matches.value_of("width") {
        println!("Value for width: {}", c);
    }

    if let Some(c) = matches.value_of("height") {
        println!("Value for height: {}", c);
    }
}
