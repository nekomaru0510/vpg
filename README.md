# Violet Project Generator (VPG)

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.txt)

VPG (Violet Project Generator) is a tool for generating project directories and boilerplate code for the [Violet Hypervisor](https://github.com/nekomaru0510/Violet). VPG makes it easy to start new multi-OS or embedded system projects using Violet by automating the creation of configuration files, source code templates, and build scripts.

## Features

- Generate new Violet project directories from a configuration file (TOML format)
- Supports multi-OS and container-based system development
- Automatically creates main.rs, setup.rs, Cargo.toml, linker scripts, and target JSON files
- Supports QEMU and custom kernel/guest OS tracing options
- Easy integration with the Violet hypervisor ecosystem

## Getting Started

### Prerequisites
- Rust toolchain (https://www.rust-lang.org/tools/install)
- [Violet Hypervisor](https://github.com/nekomaru0510/Violet) (as a dependency)
- QEMU (optional, for emulation)

### Build

```sh
cargo build --release
```

### Usage

```sh
cargo run -- -n <ProjectName> [--qemu] <config.toml>
```

- `-n`, `--name` : (required) Project name
- `-q`, `--qemu`: (optional) Enable QEMU support
- `<config.toml>`: (required) Path to the configuration file (TOML format)

#### Example

```sh
cargo run -- -n my_project sample.toml
```

This will generate a new Violet project in `../proj/my_project` based on the settings in `sample.toml`.

## Configuration File

The configuration file should be written in TOML format. See `sample.toml` for an example.

## Directory Structure

A generated project will have the following structure:

```
my_project/
  ├── Cargo.toml
  ├── src/
  │   ├── main.rs
  │   └── setup.rs
  ├── .cargo/
  │   └── config.toml
  ├── target.json
  └── target.ld
```

## Contribution

Contributions, issues, and feature requests are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/nekomaru0510/Violet).

## License

This software is released under the MIT License. See [LICENSE.txt](LICENSE.txt) for details.