use std::fs;
use std::path::PathBuf;
use cr/// Generate trap handlers for VM
fn generate_vm_trap_handlers(vm: &VmConfig, vm_name: &str) -> String {
    let mut handlers = String::new();
    
    let vm_type = vm_name.strip_prefix("vm_").unwrap_or(vm_name);
    
    // ecall handler
    handlers.push_str(&format!(
r#"pub fn do_ecall_from_vsmode_{vm_type}(sp: *mut usize) {{"#, vm_type = vm_type));ram::{SystemConfig, VmConfig};

/// Generate VM-specific module content
pub fn create_vm_module_content(vm: &VmConfig, vm_name: &str, _system: &SystemConfig) -> String {
    let mut content = String::new();
    
    // Generate imports for VM module
    content.push_str(&generate_vm_imports());
    
    // Generate trap handlers for VM
    content.push_str(&generate_vm_trap_handlers(vm, vm_name));
    
    // Generate VM boot function
    content.push_str(&generate_vm_boot_function(vm, vm_name));
    
    content
}

/// Generate imports for VM module
fn generate_vm_imports() -> String {
    let mut imports = String::new();
    
    imports.push_str("use violet::library::vm::vdev::vplic::VPlic;\n");
    imports.push_str("use violet::library::vm::{create_virtual_machine, get_mut_virtual_machine};\n\n");
    
    imports.push_str("use violet::arch::rv64::extension::hypervisor::Hext;\n");
    imports.push_str("use violet::arch::rv64::instruction::load::Load;\n");
    imports.push_str("use violet::arch::rv64::instruction::store::Store;\n");
    imports.push_str("use violet::arch::rv64::instruction::*;\n");
    imports.push_str("use violet::arch::rv64::regs::*;\n");
    imports.push_str("use violet::arch::rv64::sbi;\n");
    imports.push_str("use violet::arch::rv64::trap::int::Interrupt;\n");
    imports.push_str("use violet::arch::rv64::trap::TrapVector;\n");
    imports.push_str("use violet::arch::rv64::vscontext::*;\n");
    imports.push_str("use violet::arch::traits::context::TraitContext;\n\n");
    
    imports.push_str("use violet::environment::resource::{get_resources, BorrowResource, ResourceType};\n\n");
    
    imports
}

/// Generate trap handlers for VM
fn generate_vm_trap_handlers(vm: &VmConfig, vm_name: &str) -> String {
    let mut handlers = String::new();
    
    let vm_type = vm_name.strip_prefix("vm_").unwrap_or(vm_name);
    
    // ecall handler
    handlers.push_str(&format!(
r#"pub fn do_ecall_from_vsmode_{vm_type}(sp: *mut usize) {{
    let regs = Registers::from(sp);
    let ext: i32 = regs.reg[A7] as i32;
    let fid: i32 = regs.reg[A6] as i32;

    match sbi::Extension::from_ext(ext) {{
        sbi::Extension::SetTimer | sbi::Extension::Timer => {{
            Hext::flush_vsmode_interrupt(Interrupt::bit(
                Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
            ));
        }}
        sbi::Extension::HartStateManagement => {{
            if fid == 0 {{
                regs.reg[A0] = 0;
                regs.reg[A1] = 0;
                regs.epc = regs.epc + 4;
                return;
            }}
        }}"#, vm_type = vm_type));

    // Add OS-specific handling
    match vm.os.as_str() {
        "linux" => {
            handlers.push_str(r#"
        sbi::Extension::SystemReset => loop {},
        _ => {}
    }

    let ret = Instruction::ecall(
        ext,
        fid,
        regs.reg[A0],
        regs.reg[A1],
        regs.reg[A2],
        regs.reg[A3],
        regs.reg[A4],
        regs.reg[A5],
    );

    regs.reg[A0] = ret.0;
    regs.reg[A1] = ret.1;

    regs.epc = regs.epc + 4;
}
"#);
        }
        _ => {
            handlers.push_str(r#"
        _ => {
            regs.reg[A0] = sbi::SbiError::NotSupported as usize;
            regs.reg[A1] = 0;
        }
    }
    regs.epc = regs.epc + 4;
}
"#);
        }
    }
    
    handlers.push_str("}\n\n");

    // Helper function for Linux
    if vm.os == "linux" {
        handlers.push_str(r#"/* [todo delete] */
fn topaddr(epc: usize) -> usize {
    if epc >= 0x1_0000_0000 {
        (epc & 0x0_ffff_ffff) + 0x1000_0000 + 0x20_0000 //after MMU start in linux
    } else {
        (epc & 0x0_ffff_ffff) + 0x1000_0000 //before MMU start in linux
    }
}

"#);
    }

    // Store page fault handler
    handlers.push_str(&format!(
r#"pub fn do_guest_store_page_fault_{vm_type}(sp: *mut usize) {{
    let regs = Registers::from(sp);
    let vm = get_mut_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
"#, vm_type = vm_type));

    if vm.os == "linux" {
        handlers.push_str(r#"    let inst = Instruction::fetch(topaddr(regs.epc));
    let val = Store::from_val(inst).store_value(regs);

    match vm.dev.write(fault_paddr, val) {
        None => {
            vm.map_guest_page(Hext::get_vs_fault_paddr() as usize);
        },
        Some(()) => {
            regs.epc = regs.epc + Instruction::len(inst);
            Hext::flush_vsmode_interrupt(Interrupt::bit(
                Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT,
            ));
        }
    }
"#);
    } else {
        handlers.push_str(r#"    let inst = Instruction::fetch(regs.epc);

    match vm.dev.write(fault_paddr, regs.reg[Store::from_val(inst).src()]) {
        false => {
            vm.map_guest_page(Hext::get_vs_fault_paddr() as usize);
        },
        true => {
            regs.epc = regs.epc + Instruction::len(inst);
            Hext::assert_vsmode_interrupt(Interrupt::bit(
                Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT,
            ));
        }
    }
"#);
    }
    
    handlers.push_str("}\n\n");

    // Load page fault handler
    handlers.push_str(&format!(
r#"pub fn do_guest_load_page_fault_{vm_type}(sp: *mut usize) {{
    let regs = Registers::from(sp);
    let vm = get_mut_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
"#, vm_type = vm_type));

    if vm.os == "linux" {
        handlers.push_str(r#"    let inst = Instruction::fetch(topaddr(regs.epc));

    match vm.dev.read(fault_paddr) {
        None => {
            vm.map_guest_page(Hext::get_vs_fault_paddr() as usize);
        },
        Some(x) => {
            regs.reg[Load::from_val(inst).dst()] = x;
            regs.epc = regs.epc + Instruction::len(inst);
        }
    }
"#);
    } else {
        handlers.push_str(r#"    let inst = Instruction::fetch(regs.epc);

    match vm.dev.read(fault_paddr) {
        None => {
            vm.map_guest_page(Hext::get_vs_fault_paddr() as usize);
        },
        Some(x) => {
            regs.reg[Load::from_val(inst).dst()] = x;
            regs.epc = regs.epc + Instruction::len(inst);
        }
    }
"#);
    }
    
    handlers.push_str("}\n\n");

    // Instruction page fault handler
    handlers.push_str(&format!(
r#"pub fn do_guest_instruction_page_fault_{vm_type}(_sp: *mut usize) {{
    let vm = get_mut_virtual_machine();
    vm.map_guest_page(Hext::get_vs_fault_paddr() as usize);
}}

"#, vm_type = vm_type));

    // External interrupt handler
    handlers.push_str(&format!(
r#"pub fn do_supervisor_external_interrupt_{vm_type}(_sp: *mut usize) {{
    let vm = get_mut_virtual_machine();
    // Read and clear the pending bit from the physical PLIC
    let int_id = if let BorrowResource::Intc(i) = get_resources().get(ResourceType::Intc, 0) {{
        i.get_pend_int()
    }} else {{
        0
    }};

    // write to virtual plic
    match vm.dev.get_mut(0x0c20_1000) {{
        None => (),
        Some(d) => {{
            d.interrupt(int_id as usize);
        }}
    }}

    // Raise a virtual external interrupt 
    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT,
    ));
"#, os = vm.os));

    if vm.os == "linux" {
        handlers.push_str(r#"
    // Clear the pending bit in the PLIC 
    if let BorrowResource::Intc(i) = get_resources().get(ResourceType::Intc, 0) {
        i.set_comp_int(int_id);
    }
"#);
    }

    handlers.push_str("}\n\n");

    // Timer interrupt handler
    handlers.push_str(&format!(
r#"pub fn do_supervisor_timer_interrupt_{vm_type}(_sp: *mut usize) {{
"#, vm_type = vm_type));

    if vm.os == "linux" {
        handlers.push_str(r#"    // Disable the timer
    sbi::sbi_set_timer(0xffff_ffff_ffff_ffff);

    // Raise a timer interrupt to the guest
    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
    ));
"#);
    } else {
        handlers.push_str(r#"    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
    ));
"#);
    }

    handlers.push_str("}\n\n");

    handlers
}

/// Generate VM boot function
fn generate_vm_boot_function(vm: &VmConfig, vm_name: &str) -> String {
    let mut content = String::new();
    
    let vm_type = vm_name.strip_prefix("vm_").unwrap_or(vm_name);
    
    // Determine boot core from CPU mappings or use default
    let boot_core = if !vm.cpus.pcpu_mapping.is_empty() {
        vm.cpus.pcpu_mapping[0]
    } else {
        0
    };

    // Determine kernel entry point based on OS
    let (kernel_entry, kernel_args) = match vm.os.as_str() {
        "linux" => ("0x8020_0000", "0x8220_0000"), // Linux kernel entry with FDT
        "freertos" => ("0x8000_0000", "0"),         // FreeRTOS entry point
        _ => ("0x8000_0000", "0"),                  // Default entry point
    };

    content.push_str(&format!(
r#"pub fn boot_{vm_type}() {{
    let boot_core = {boot_core};
    
    /* Setup virtual machine */
    create_virtual_machine();
    let vm = get_mut_virtual_machine();
    vm.reset();

    /* CPU */
    vm.cpu.register(0, boot_core); /* vcpu0 ... pcpu{boot_core} */
    match vm.cpu.get_mut(0) {{
        None => (),
        Some(v) => {{
            v.context.set(JUMP_ADDR, {kernel_entry});
            v.context.set(ARG0, 0);
            v.context.set(ARG1, {kernel_args});
        }}
    }}

    /* RAM */
"#, 
        vm_type = vm_type,
        boot_core = boot_core,
        kernel_entry = kernel_entry,
        kernel_args = kernel_args
    ));

    // Generate memory mappings
    if vm.memory.is_empty() {
        match vm.os.as_str() {
            "linux" => {
                content.push_str("    vm.mem.register(0x8020_0000, 0x9020_0000, 0x1000_0000);\n");
                content.push_str("    vm.mem.register(0x8220_0000, 0x8220_0000, 0x2_0000);    // FDT is mapped to physical memory.\n");
                content.push_str("    vm.mem.register(0x8810_0000, 0x8810_0000, 0x20_0000);    // initrd is also mapped to physical memory. The size is estimated from rootfs.img\n");
            }
            "freertos" => {
                content.push_str("    vm.mem.register(0x80000000, 0x80000000, 0x20000000);\n");
            }
            _ => {
                content.push_str("    vm.mem.register(0x80000000, 0x80000000, 0x20000000);\n");
            }
        }
    } else {
        for memory in &vm.memory {
            content.push_str(&format!(
                "    vm.mem.register(0x{:x}, 0x{:x}, 0x{:x});\n",
                memory.guest_base, memory.host_base, memory.size
            ));
        }
    }
    
    content.push_str("    vm.mmu_enable();\n\n");

    // Generate virtual devices
    content.push_str("    /* MMIO */\n");
    if vm.vdev.is_empty() {
        content.push_str(&format!(
r#"    let mut vplic = VPlic::new();
    vplic.set_vcpu_config([boot_core, 0]); /* vcpu0 ... pcpu{boot_core} */
    vm.dev.register(0x0c00_0000, 0x0400_0000, vplic);
"#,
            boot_core = boot_core
        ));
    } else {
        for vdev in &vm.vdev {
            match vdev.device.as_str() {
                "vplic" | "plic" => {
                    content.push_str(&format!(
r#"    let mut vplic = VPlic::new();
    vplic.set_vcpu_config([{boot_core}, 0]); /* vcpu0 ... pcpu{boot_core} */
    vm.dev.register(0x{guest_base:x}, 0x{size:x}, vplic);
"#,
                        boot_core = boot_core,
                        guest_base = vdev.guest_base,
                        size = vdev.size
                    ));
                }
                _ => {
                    content.push_str(&format!(
                        "    // TODO: Implement {} device at 0x{:x}\n",
                        vdev.device, vdev.guest_base
                    ));
                }
            }
        }
    }

    // Register trap handlers
    content.push_str(&format!(
r#"    
    /* Register interrupt/exception handler */
    if vm.trap.register_traps(
        &[
            (TrapVector::SUPERVISOR_TIMER_INTERRUPT, do_supervisor_timer_interrupt_{vm_type}),
            (TrapVector::SUPERVISOR_EXTERNAL_INTERRUPT, do_supervisor_external_interrupt_{vm_type}),
            (TrapVector::ENVIRONMENT_CALL_FROM_VSMODE, do_ecall_from_vsmode_{vm_type}),
            (TrapVector::LOAD_GUEST_PAGE_FAULT, do_guest_load_page_fault_{vm_type}),
            (TrapVector::STORE_AMO_GUEST_PAGE_FAULT, do_guest_store_page_fault_{vm_type}),
            (TrapVector::INSTRUCTION_GUEST_PAGE_FAULT, do_guest_instruction_page_fault_{vm_type}),
        ]
    ) == Err(()) {{ panic!("Fail to register trap"); }}

    /* Run */
    vm.run();
}}
"#,
        vm_type = vm_type
    ));

    content
}

/// Generate VM module file
pub fn generate_vm_module_file(path: &String, vm: &VmConfig, vm_name: &str, system: &SystemConfig) {
    let vm_module_content = create_vm_module_content(vm, vm_name, system);
    let vm_type = vm_name.strip_prefix("vm_").unwrap_or(vm_name);
    let vm_module_path = PathBuf::from(path).join("src").join(format!("{}.rs", vm_type));
    fs::write(&vm_module_path, vm_module_content).unwrap();
}
