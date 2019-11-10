extern crate clap;

use clap::{App, Arg, ArgMatches};

fn main() {

    let matches = App::new("conswitch")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("verbose")
            .help("guess what")
            .short("v")
            .long("verbose")
        )
        .get_matches();

    debug("verbose is set to on!", matches);

    println!("Nothing to see for now, whoopsie!");
}

/// Prints out to console, if --verbose has been used
fn debug(text: &str, matches: ArgMatches) {
    if matches.is_present("verbose") {
        println!("{}", text);
    }
}

