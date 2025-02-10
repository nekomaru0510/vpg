#![feature(trivial_bounds)] // todo remove

extern crate serde;
extern crate toml;
extern crate getopts;

mod reader;
mod format;
mod param;
mod generator;

use std::env;
use getopts::Options;
use crate::param::{SystemConfig, ContainerConfig};
use crate::format::toml::Toml;
use crate::format::Parser;
use crate::generator::Generator;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] <TOML file>", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    // //////////////////////////
    // Parse command line options
    // //////////////////////////

    let mut opts = Options::new();
    opts.optopt("n", "name", "set project name", "NAME");
    opts.optflag("q", "qemu", "support QEMU execution");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}", f.to_string());
            print_usage(program, opts);
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(program, opts);
        return Ok(());
    }

    let project_name = match matches.opt_str("n") {
        Some(name) => name,
        None => {
            eprintln!("Error: Project name is required.");
            print_usage(program, opts);
            std::process::exit(1);
        }
    };
    
    let qemu_support = matches.opt_present("q");

    if matches.free.len() != 1 {
        eprintln!("Error: TOML file is required.");
        print_usage(program, opts);
        std::process::exit(1);
    }

    let toml_file = &matches.free[0];

    // //////////////////////////
    // Main process
    // //////////////////////////

    // Read configuration file
    let tomlcfg = reader::read_file(toml_file)?;

    // Parse configuration file
    let system = Parser::parse::<Toml>(tomlcfg)?;

    // Check configuration

    // Generate project
    Generator::generate(&project_name, &system);

    Ok(())
}
