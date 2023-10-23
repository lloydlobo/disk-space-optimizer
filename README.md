# Disk Space Optimizer

![GitHub](https://img.shields.io/github/license/lloydlobo/disk-space-optimizer)
![GitHub](https://img.shields.io/github/stars/lloydlobo/disk-space-optimizer?style=social)

**Disk Space Optimizer** is a powerful and flexible command-line tool designed
to help you efficiently manage and optimize disk space on your Linux system.

Whether you want to remove unnecessary packages, clean package caches, uninstall
unused applications, remove old kernel versions, or clean up log files,
this CLI has got you covered.

<!--toc:start-->
- [Disk Space Optimizer](#disk-space-optimizer)
  - [Installation](#installation)
  - [Build](#build)
  - [Usage](#usage)
  - [Available Commands](#available-commands)
  - [Examples](#examples)
  - [Contributing](#contributing)
  - [License](#license)
<!--toc:end-->


## Installation

Before using Disk Space Optimizer, ensure that you have Rust and Cargo installed on your system. If you don't have them already, you can easily set them up with Rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once you have Rust and Cargo installed, you can proceed with the installation of the CLI.

1. Clone the repository:

```bash
git clone https://github.com/lloydlobo/disk-space-optimizer.git
```

2. Change your working directory to the project folder:

```bash
cd disk-space-optimizer
```

3. Build the CLI:

```bash
cargo build --release
```

Now, you're all set to use Disk Space Optimizer.

## Build

**Prerequisite:** Ensure that you have added `$HOME/.cargo/bin` to your `PATH` in your `.bashrc` file.

To install the built binary, use the following commands:

```bash
cargo build --release
cargo install --path .
```

## Usage

If the binary is installed in your PATH, you can omit `cargo run --bin` from
the following commands. Running `disk-space-optimizer` from anywhere in
your terminal will be sufficient. Otherwise, you need to be in this
repository 's directory to execute the commands.

The CLI is executed using the following command:

```bash
cargo run --bin disk-space-optimizer
```

## Available Commands

Disk Space Optimizer provides the following commands for disk space optimization:

1. **Remove Unnecessary Packages**: This command allows you to remove unnecessary packages from your system. You will be prompted to enter the name of the package you want to remove.

2. **Clean Package Cache**: Use this command to clean the package cache. It helps in freeing up disk space occupied by cached packages.

3. **Uninstall Unused Applications**: This command will uninstall unused applications from your system, freeing up valuable disk space.

4. **Remove Old Kernel Versions**: If you have multiple kernel versions installed, you can use this command to remove older, unused kernel versions. You will be prompted to select the versions to remove.

5. **Clean Up Log Files**: This command helps in cleaning up log files, which can consume disk space over time. You can specify the number of days to retain logs.

```shell
$ disk-space-optimizer --help
A CLI tool for optimizing disk space

Usage: disk-space-optimizer [COMMAND]

Commands:
  remove-package         Removes a package with the specified name
  clean-package-cache    Cleans the package cache
  uninstall-unused-apps  Uninstalls unused apps
  remove-old-kernels     Removes old kernels
  clean-up-log-files     Cleans up log files
  help                   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

Here are a few examples of how to use Disk Space Optimizer:

- To remove unnecessary packages:

  ```bash
  cargo run --bin disk-space-optimizer remove-package
  ```

  Follow the prompts to enter the package name for removal.

- To clean the package cache:

  ```bash
  cargo run --bin disk-space-optimizer clean-package-cache
  ```

- To uninstall unused applications:

  ```bash
  cargo run --bin disk-space-optimizer uninstall-unused-apps
  ```

- To remove old kernel versions:

  ```bash
  cargo run --bin disk-space-optimizer remove-old-kernels
  ```

  Follow the prompts to select the kernel versions to remove.

- To clean up log files:

  ```bash
  cargo run --bin disk-space-optimizer clean-up-log-files
  ```

  Specify the number of days to retain logs.

## Contributing

We welcome contributions! If you have ideas for improvements, new features,
or bug fixes, please open an issue or submit a pull request on
[GitHub](https://github.com/lloydlobo/disk-space-optimizer).

## License

This CLI is open-source and available under the [MIT License](LICENSE).
You are free to use it, modify it, and distribute it as per the terms of the license.

Enjoy optimizing your disk space with Disk Space Optimizer!
