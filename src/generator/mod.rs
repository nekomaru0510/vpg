use std::fs;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;
use std::process::Stdio;
use crate::param::SystemConfig;

pub mod gen_main;
pub mod gen_vm;

pub struct Generator {}

impl Generator {
    pub fn generate(name: &String, system: &SystemConfig) {
        
        // [todo fix] Path should be specified via command line arguments
        let path: String = "../../proj/".to_string() + name;

        if file_exists(&path) {
            println!("Error: {} already exists", path);
            return;
        }

        // cargo new
        generate_project(&path);

        // add main.rs and setup.rs
        generate_source(&path, system);
        
        // modify Cargo.toml
        modify_cargo_toml(&path, system);

        // add .cargo/config.toml
        generate_config_toml(&path, system);
        
    }
}

fn generate_project(name: &String) {
    let output = Command::new("cargo")
        .args(["new", name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output() // コマンドを実行
        .unwrap();
    //println!("{:?}", output);
    //println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn generate_setup_rs(path: &String, system: &SystemConfig) {
    
}

// File extension utilities to avoid string literal parsing issues
fn make_template_dir() -> String { "template".to_string() }
fn make_cargo_toml() -> String { format!("Cargo.{}", "toml") }
fn make_cargo_dir() -> String { format!(".{}", "cargo") }
fn make_config_toml() -> String { format!("config.{}", "toml") }
fn make_target_json() -> String { format!("target.{}", "json") }
fn make_target_ld() -> String { format!("target.{}", "ld") }
fn make_setup_rs() -> String { format!("setup.{}", "rs") }

fn generate_source(path: &String, system: &SystemConfig) {
    // Generate main.rs using SystemConfig
    gen_main::generate_main_rs(path, system);

    // Generate VM modules for each VM
    for (vm_name, vm) in &system.vms {
        gen_vm::generate_vm_module_file(path, vm, vm_name, system);
    }

    // Generate setup.rs with dynamic CPU configuration
    use std::path::PathBuf;
    let template_path = PathBuf::from(make_template_dir()).join(make_setup_rs());
    let content = replace_from_template(template_path.to_str().unwrap(), &[("{{NUM_OF_CPUS}}", &system.environment.num_of_cpus.to_string())]);
    let setup_path = PathBuf::from(path).join("src").join(make_setup_rs());
    fs::write(&setup_path, content).unwrap();
}

fn modify_cargo_toml(path: &String, system: &SystemConfig) {
    use std::path::PathBuf;
    let cargo_path = PathBuf::from(path).join(make_cargo_toml());
    let violet_dep = format!("violet = {{ path = \"{}\" }}", "../../violet");
    let _ = append_to_file(cargo_path.to_str().unwrap(), &violet_dep);
}

fn generate_config_toml(path: &String, system: &SystemConfig) {
    use std::path::PathBuf;
    
    let cargo_dir = PathBuf::from(path).join(make_cargo_dir());
    fs::create_dir_all(&cargo_dir).unwrap();
    
    // Copy config.toml
    let config_src = PathBuf::from(make_template_dir()).join(make_cargo_dir()).join(make_config_toml());
    let config_dst = cargo_dir.join(make_config_toml());
    fs::copy(config_src, &config_dst).unwrap();
    
    // Copy target.json
    let json_src = PathBuf::from(make_template_dir()).join(make_target_json());
    let json_dst = PathBuf::from(path).join(make_target_json());
    fs::copy(json_src, &json_dst).unwrap();
    
    // Copy target.ld
    let ld_src = PathBuf::from(make_template_dir()).join(make_target_ld());
    let ld_dst = PathBuf::from(path).join(make_target_ld());
    fs::copy(ld_src, &ld_dst).unwrap();
}

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn append_to_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)?;

    file.write_all(content.as_bytes())?;
    const NEWLINE: &[u8] = &[10]; // ASCII newline
    file.write_all(NEWLINE)?;
    Ok(())
}

fn replace_from_template(template_path: &str, replacements: &[(&str, &str)]) -> String {
    let mut content = fs::read_to_string(template_path).unwrap();
    for &(placeholder, replacement) in replacements {
        content = content.replace(placeholder, replacement);
    }
    content
}

#[derive(Debug)]
pub enum GeneratorError {
    GenerateError(String),
}

