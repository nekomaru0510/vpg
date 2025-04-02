extern crate serde;
extern crate toml;

use serde::{Deserialize, Serialize};
use super::{TraitFormat, FormatError};
use crate::param::{SystemConfig, ContainerConfig};

pub struct Toml {
    text: String,
    table: Option<toml::Table>
}

impl TraitFormat for Toml {
    fn parse(text: String) -> Result<SystemConfig, FormatError> {
        match text.parse::<toml::Value>() {
            Ok(t) => inner_parse(t),
            Err(e) => {
                println!("{}", e);
                Err(FormatError::ParseError(format!("Failed to parse TOML")))
            }
            
        }
    }
}


fn inner_parse(table: toml::Value) -> Result<SystemConfig, FormatError> {
    let version = table.get("version").unwrap().as_float().unwrap();
    let num_of_cpus = 2;//table.get("env.num_of_cpus").unwrap().as_integer().unwrap();
    let num_of_containers = 1;

    let mut container_configs = Vec::new();
    for i in 1 .. num_of_containers+1 {
        container_configs.push(parse_container(i, table.clone()));
    }

    Ok(SystemConfig {
        version: version as f32,
        num_of_cpus: num_of_cpus as u64,
        num_of_containers: num_of_containers as u64,
        containers: container_configs,
    })
}

fn parse_container(id: u64, container: toml::Value) -> ContainerConfig {
    
    let mut num_of_cpus: u64 = 0;
    
    if let Some(env) = container.get("env") {
        if let Some(num_of_cpus) = env.get("num_of_cpus") {
            let bsp = env.get("bsp").unwrap().as_integer().unwrap();
            let cores = 3;//env.get("cores").unwrap().as_integer().unwrap();
            return ContainerConfig {
                id: id,
                num_of_cpus: num_of_cpus.as_integer().unwrap() as u64,
                bsp: bsp as u64,
                cores: cores as u64,
            }
        }
    }
    
    let bsp = container.get("env.bsp").unwrap().as_integer().unwrap();
    let cores = 3;//container.get("cores").unwrap().as_integer().unwrap();
    ContainerConfig {
        id: id,
        num_of_cpus: num_of_cpus as u64,
        bsp: bsp as u64,
        cores: cores as u64,
    }
}