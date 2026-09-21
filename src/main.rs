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
use crate::format::toml::Toml;
use crate::format::Parser;
use crate::generator::Generator;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] <TOML file>", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Check for test mode
    if args.len() > 1 && args[1] == "test" {
        test_parser();
        return Ok(());
    }
    
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

fn test_parser() {
    use std::fs;
    
    println!("Testing VPG TOML Parser...");
    
    let content = fs::read_to_string("test_config.toml")
        .expect("Failed to read test configuration file");
    
    match Parser::parse::<Toml>(content) {
        Ok(config) => {
            println!("✓ Successfully parsed TOML configuration");
            println!("  Version: {}", config.version);
            println!("  Name: {}", config.name);
            if let Some(test) = config.test {
                println!("  Test mode: {}", test);
            }
            println!("  VMs: {}", config.vms.len());
            println!("  Environment Arch: {}", config.environment.arch);
            println!("  Environment CPUs: {}", config.environment.num_of_cpus);
            println!("  Physical Memory Base: 0x{:x}", config.environment.memory.base);
            println!("  Physical Memory Size: 0x{:x}", config.environment.memory.size);
            println!("  Devices: {}", config.environment.devices.len());
            
            for (vm_name, vm) in &config.vms {
                println!("  VM '{}': {}", vm_name, vm.os);
                if let Some(version) = &vm.os_version {
                    println!("    OS Version: {}", version);
                }
                println!("    CPUs: {}", vm.cpus.num_cpus);
                println!("    Memory Mappings: {}", vm.memory.len());
                println!("    Virtual Devices: {}", vm.vdev.len());
            }
            
            if let Some(scheduler) = &config.kernel.scheduler {
                println!("  Kernel Scheduler: {}", scheduler);
            }
        }
        Err(e) => {
            println!("✗ Failed to parse TOML configuration: {:?}", e);
        }
    }
}
