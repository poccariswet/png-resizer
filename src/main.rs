extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("image resizer")
        .version("1.0")
        .author("poccariswet <poccariswet2727@gmail.com>")
        .about("Does awesome things")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'
                              <INPUT>              'Sets the input file to use'
                              -v...                'Sets the level of verbosity'",
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg("-d, --debug 'Print debug information'"),
        )
        .get_matches();
}
