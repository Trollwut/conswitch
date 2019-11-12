extern crate pretty_env_logger;
#[macro_use] extern crate log;
extern crate config;
extern crate dirs;

use std::env;
//use std::path::PathBuf;
use structopt::StructOpt;
use log::{info, warn, error};
use config::*;

#[derive(StructOpt, Debug)]
struct Params {
    /// Verbose mode (-v, -vv, -vvv are possible)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

fn main() {

    let params = Params::from_args();

    set_env_log(params);
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let mut config = Config::default();
    //let mut config_dir: PathBuf = dirs::config_dir();

    //println!("{:?}", dirs::config_dir());

    println!("Nothing to see for now, whoopsie!");

    info!("such information");
    warn!("o_O");
    error!("boom");
    debug!("deboogging");

    // cleanup before exiting
    env::remove_var("RUST_APP_LOG");
}

/// depending on the amount of --verbose, this sets the level of logging to console
fn set_env_log(params: Params) {
    let envlevel;
    println!("{}", params.verbose);
    match params.verbose {
        0 => envlevel = "error",
        1 => envlevel = "warn",
        2 => envlevel = "info",
        3 | _ => envlevel = "trace",
    }
    env::set_var("RUST_APP_LOG", envlevel);
}
