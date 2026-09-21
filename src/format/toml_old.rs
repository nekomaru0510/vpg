extern crate serde;
extern crate toml;

use serde::{Deserialize, Serialize};
use super::{TraitFormat, FormatError};
use crate::param::*;

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
    let version = table.get("version")
        .and_then(|v| v.as_float())
        .unwrap_or(0.6) as f32;

    let name = table.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Default Project")
        .to_string();

    let test = table.get("test")
        .and_then(|v| v.as_bool());

    // Parse VMs (all vm_* sections)
    let vms = parse_vms(&table)?;

    // Parse environment config
    let environment = parse_environment(&table)?;
    let kernel = parse_kernel(&table)?;

    Ok(SystemConfig {
        version,
        name,
        test,
        vms,
        environment,
        kernel,
    })
}

fn parse_vms(table: &toml::Value) -> Result<Vec<(String, VmConfig)>, FormatError> {
    let mut vms = Vec::new();
    
    if let Some(table_map) = table.as_table() {
        for (key, value) in table_map {
            if key.starts_with("vm_") {
                let vm_config = parse_vm_config(value)?;
                vms.push((key.clone(), vm_config));
            }
        }
    }
    
    Ok(vms)
}

fn parse_vm_config(vm_table: &toml::Value) -> Result<VmConfig, FormatError> {
    let os = vm_table.get("os")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    
    let os_version = vm_table.get("os_version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    // Parse CPU config
    let cpus = parse_cpu_config(vm_table)?;
    
    // Parse memory mappings
    let memory = parse_memory_mappings(vm_table)?;
    
    // Parse virtual devices
    let vdev = parse_vdev(vm_table)?;

    Ok(VmConfig {
        os,
        os_version,
        cpus,
        memory,
        vdev,
    })
}

fn parse_cpu_config(vm_table: &toml::Value) -> Result<CpuConfig, FormatError> {
    let cpu_section = vm_table.get("cpus")
        .ok_or(FormatError::ParseError("cpus section not found".to_string()))?;
    
    let num_cpus = cpu_section.get("num_cpus")
        .and_then(|v| v.as_integer())
        .unwrap_or(1) as u64;
    
    let pcpu_mapping = cpu_section.get("pcpu_mapping")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_integer()).map(|i| i as u64).collect())
        .unwrap_or_else(|| (0..num_cpus).collect());

    Ok(CpuConfig {
        num_cpus,
        pcpu_mapping,
    })
}

fn parse_memory_mappings(vm_table: &toml::Value) -> Result<Vec<MemoryMapping>, FormatError> {
    let mut memory = Vec::new();
    
    if let Some(mem_array) = vm_table.get("memory") {
        if let Some(mem_list) = mem_array.as_array() {
            for mem_value in mem_list {
                let guest_base = mem_value.get("guest_base")
                    .and_then(|v| v.as_integer())
                    .unwrap_or(0) as u64;
                
                let host_base = mem_value.get("host_base")
                    .and_then(|v| v.as_integer())
                    .unwrap_or(0) as u64;
                
                let size = mem_value.get("size")
                    .and_then(|v| v.as_integer())
                    .unwrap_or(0) as u64;
                
                memory.push(MemoryMapping { guest_base, host_base, size });
            }
        }
    }
    
    Ok(memory)
}

fn parse_vdev(vm_table: &toml::Value) -> Result<Vec<VDevConfig>, FormatError> {
    let mut vdev = Vec::new();
    
    if let Some(vdev_array) = vm_table.get("vdev") {
        if let Some(vdev_list) = vdev_array.as_array() {
            for vdev_value in vdev_list {
                let device = vdev_value.get("device")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                let guest_base = vdev_value.get("guest_base")
                    .and_then(|v| v.as_integer())
                    .unwrap_or(0) as u64;
                
                let size = vdev_value.get("size")
                    .and_then(|v| v.as_integer())
                    .unwrap_or(0) as u64;
                
                let config = vdev_value.get("config").cloned();
                
                vdev.push(VDevConfig { device, guest_base, size, config });
            }
        }
    }
    
    Ok(vdev)
}

fn parse_environment(table: &toml::Value) -> Result<EnvironmentConfig, FormatError> {
    let env = table.get("env").ok_or(FormatError::ParseError("env section not found".to_string()))?;
    
    let arch = env.get("arch")
        .and_then(|v| v.as_str())
        .unwrap_or("rv64")
        .to_string();
    
    let hext = env.get("hext")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    let num_of_cpus = env.get("num_of_cpus")
        .and_then(|v| v.as_integer())
        .unwrap_or(2) as u64;
    
    let cores = env.get("cores")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_integer()).map(|i| i as u64).collect())
        .unwrap_or_else(|| (0..num_of_cpus).collect());
    
    let bsp = env.get("bsp")
        .and_then(|v| v.as_integer())
        .unwrap_or(0) as u64;

    // Parse memory config
    let memory = parse_physical_memory(env)?;
    
    // Parse device configs
    let devices = parse_devices(env)?;

    // Parse QEMU config (optional)
    let qemu = parse_qemu_config(env);

    Ok(EnvironmentConfig {
        arch,
        hext,
        num_of_cpus,
        cores,
        bsp,
        memory,
        devices,
        qemu,
    })
}

fn parse_qemu_config(env: &toml::Value) -> Option<QemuConfig> {
    env.get("qemu").map(|qemu_section| {
        let kernel_path = qemu_section.get("kernel_path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let initrd_path = qemu_section.get("initrd_path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let dtb_path = qemu_section.get("dtb_path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let bios_path = qemu_section.get("bios_path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        QemuConfig {
            kernel_path,
            initrd_path,
            dtb_path,
            bios_path,
        }
    })
}

fn parse_physical_memory(env: &toml::Value) -> Result<PhysicalMemoryConfig, FormatError> {
    let memory_section = env.get("memory")
        .ok_or(FormatError::ParseError("env.memory section not found".to_string()))?;
    
    let base = memory_section.get("base")
        .and_then(|v| v.as_integer())
        .unwrap_or(0x80000000) as u64;
    
    let size = memory_section.get("size")
        .and_then(|v| v.as_integer())
        .unwrap_or(0x40000000) as u64;

    Ok(PhysicalMemoryConfig { base, size })
}

fn parse_devices(env: &toml::Value) -> Result<Vec<DeviceConfig>, FormatError> {
    let mut devices = Vec::new();
    
    if let Some(device_array) = env.get("device") {
        if let Some(device_list) = device_array.as_array() {
            for device_value in device_list {
                let device = device_value.get("device")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                let base = device_value.get("base")
                    .and_then(|v| v.as_integer())
                    .unwrap_or(0) as u64;
                
                let size = device_value.get("size")
                    .and_then(|v| v.as_integer())
                    .unwrap_or(0) as u64;
                
                devices.push(DeviceConfig { device, base, size });
            }
        }
    }
    
    Ok(devices)
}

fn parse_kernel(table: &toml::Value) -> Result<KernelConfig, FormatError> {
    let kernel_section = table.get("kernel");
    
    let scheduler = kernel_section
        .and_then(|k| k.get("scheduler"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let dispatcher = kernel_section
        .and_then(|k| k.get("dispatcher"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let heap_size = kernel_section
        .and_then(|k| k.get("heap_size"))
        .and_then(|v| v.as_integer())
        .map(|i| i as u64);
    
    let stack_size = kernel_section
        .and_then(|k| k.get("stack_size"))
        .and_then(|v| v.as_integer())
        .map(|i| i as u64);

    Ok(KernelConfig {
        scheduler,
        dispatcher,
        heap_size,
        stack_size,
    })
}
        num_of_cpus,
        cores,
        bsp,
        memory,
        devices,
    })
}

fn parse_physical_memory(env: &toml::Value) -> Result<PhysicalMemoryConfig, FormatError> {
    if let Some(mem) = env.get("memory") {
        let base = mem.get("base")
            .and_then(|v| v.as_integer())
            .unwrap_or(0x80000000) as u64;
        
        let size = mem.get("size")
            .and_then(|v| v.as_integer())
            .unwrap_or(0x40000000) as u64;
        
        Ok(PhysicalMemoryConfig { base, size })
    } else {
        Ok(PhysicalMemoryConfig { base: 0x80000000, size: 0x40000000 })
    }
}

fn parse_devices(env: &toml::Value) -> Result<Vec<DeviceConfig>, FormatError> {
    let mut devices = Vec::new();
    
    if let Some(dev_array) = env.get("device") {
        if let Some(dev_list) = dev_array.as_array() {
            for dev_value in dev_list {
                let device = dev_value.get("device")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                let base = dev_value.get("base")
                    .and_then(|v| v.as_integer())
                    .unwrap_or(0) as u64;
                
                let size = dev_value.get("size")
                    .and_then(|v| v.as_integer())
                    .unwrap_or(0) as u64;
                
                devices.push(DeviceConfig { device, base, size });
            }
        }
    }
    
    Ok(devices)
}

fn parse_kernel(table: &toml::Value) -> Result<KernelConfig, FormatError> {
    let kernel = table.get("kernel");
    
    let template = kernel
        .and_then(|k| k.get("template"))
        .and_then(|v| v.as_str())
        .unwrap_or("rtkernel")
        .to_string();
    
    let scheduler = kernel
        .and_then(|k| k.get("scheduler"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let dispatcher = kernel
        .and_then(|k| k.get("dispatcher"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let heap_size = kernel
        .and_then(|k| k.get("heap_size"))
        .and_then(|v| v.as_integer())
        .map(|i| i as u64);
    
    let stack_size = kernel
        .and_then(|k| k.get("stack_size"))
        .and_then(|v| v.as_integer())
        .map(|i| i as u64);

    Ok(KernelConfig {
        template,
        scheduler,
        dispatcher,
        heap_size,
        stack_size,
    })
}