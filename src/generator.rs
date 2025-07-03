use std::fs;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;
use std::process::Stdio;
use crate::param::{SystemConfig, ContainerConfig};

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

fn generate_source(path: &String, system: &SystemConfig) {

    fs::copy("template/main.rs", &(path.to_string()+"/src/main.rs")).unwrap();

    //fs::copy("template/setup.rs", &(path.to_string()+"/src/setup.rs")).unwrap();
    let content = replace_from_template("template/setup.rs", &[("{{NUM_OF_CPUS}}", "12")]);
    fs::write(&(path.to_string()+"/src/setup.rs"), content).expect("Failed to write output file");
}

fn modify_cargo_toml(path: &String, system: &SystemConfig) {
    append_to_file(&(path.to_string()+"/Cargo.toml"), "violet = { path = '../../violet' }");
}

fn generate_config_toml(path: &String, system: &SystemConfig) {
    fs::create_dir_all(&(path.to_string()+"/.cargo")).unwrap();
    fs::copy("template/.cargo/config.toml", &(path.to_string()+"/.cargo/config.toml")).unwrap();
    fs::copy("template/target.json", &(path.to_string()+"/target.json")).unwrap(); 
    fs::copy("template/target.ld", &(path.to_string()+"/target.ld")).unwrap();
}

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn append_to_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)?;

    writeln!(file, "{}", content)?;
    Ok(())
}

fn replace_from_template(template_path: &str, replacements: &[(&str, &str)]) -> String {
    let mut content = fs::read_to_string(template_path).expect("Failed to read template");
    for &(placeholder, replacement) in replacements {
        content = content.replace(placeholder, replacement);
    }
    content
}

#[derive(Debug)]
pub enum GeneratorError {
    GenerateError(String),
}

