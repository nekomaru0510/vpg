#[derive(Debug)]
pub struct SystemConfig {
    pub version: f32,
    pub num_of_cpus: u64,
    pub num_of_containers: u64,
    pub containers: Vec<ContainerConfig>,
}

#[derive(Debug)]
pub struct ContainerConfig {
    pub id: u64,
    pub num_of_cpus: u64,
    pub bsp: u64,
    pub cores: u64,
}