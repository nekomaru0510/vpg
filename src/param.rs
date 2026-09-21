#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub version: f32,
    pub name: String,
    pub test: Option<bool>,
    pub vms: Vec<(String, VmConfig)>,  // VM名とVMConfigのペア
    pub environment: EnvironmentConfig,
    pub kernel: KernelConfig,
}

#[derive(Debug, Clone)]
pub struct VmConfig {
    pub os: String,
    pub os_version: Option<String>,
    pub cpus: CpuConfig,
    pub memory: Vec<MemoryMapping>,
    pub vdev: Vec<VDevConfig>,
}

#[derive(Debug, Clone)]
pub struct CpuConfig {
    pub num_cpus: u64,
    pub pcpu_mapping: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct MemoryMapping {
    pub guest_base: u64,
    pub host_base: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct VDevConfig {
    pub device: String,
    pub guest_base: u64,
    pub size: u64,
    pub config: Option<toml::Value>,
}

#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    pub arch: String,
    pub hext: bool,
    pub num_of_cpus: u64,
    pub cores: Vec<u64>,
    pub bsp: u64,
    pub memory: PhysicalMemoryConfig,
    pub devices: Vec<DeviceConfig>,
    pub qemu: Option<QemuConfig>,
}

#[derive(Debug, Clone)]
pub struct QemuConfig {
    pub kernel_path: Option<String>,
    pub initrd_path: Option<String>,
    pub dtb_path: Option<String>,
    pub bios_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PhysicalMemoryConfig {
    pub base: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct DeviceConfig {
    pub device: String,
    pub base: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct KernelConfig {
    pub scheduler: Option<String>,
    pub dispatcher: Option<String>,
    pub heap_size: Option<u64>,
    pub stack_size: Option<u64>,
}