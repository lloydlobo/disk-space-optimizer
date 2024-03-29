//! # Disk Space Optimizer
//!
//! This is a command-line interface (CLI) for the disk space optimizer.
//! It allows the user to perform disk space optimization operations, such as on a Linux system.
//!
//! # Usage
//!
//! The CLI is executed using the following command:
//!
//! ```bash
//! cargo run --bin disk-space-optimizer
//! ```

#![deny(missing_docs)]

#[cfg(test)]
mod tests;

use std::{
    env::consts::OS,
    io::{self, prelude::*},
    process::Command,
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;

/// The main function of the disk space optimizer CLI. Parses the command-line arguments using
/// `Cli::parse()`, then displays the welcome message and presents a menu of options to the user.
///
/// If a command argument was passed in the command line, that command is executed. Otherwise, the
/// user is presented with a menu of options to select from. The `run_dialoguer` function from the
/// `multidialogue` crate is used to display the menu and capture the user's selections.
///
/// # Examples
///
/// ```
/// use anyhow::Result;
/// use disk_space_optimizer::main;
///
/// fn run() -> Result<()> {
///     main()
/// }
/// ```
fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let current_os = OS;
    println!("Welcome to disk space optimizer CLI for {current_os}!",);

    match &cli.command {
        Some(command) => command.execute()?,
        _ => {
            let commands = get_commands();
            let selections = multidialogue::run_dialoguer(&commands)?;

            for selection in selections.into_iter() {
                if let Some(command) = cli::Commands::from_selection(selection.key as usize) {
                    if let Err(err) = command.execute() {
                        println!("Error: {err}", err = anyhow!(err));
                    }
                }
            }
        }
    }
    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// REGION_START: helper functions
//////////////////////////////////////////////////////////////////////////////////////////////////

/// The `get_commands` function returns a `DiskSpaceOptimizerItems` object that contains a set of
/// options for disk space optimization.
///
/// Each option is represented as a `SelectableItem`, which is a struct containing a numeric ID and
/// a string label.
///
/// The available options are:
///
/// 1. "Remove unnecessary packages"
/// 2. "Clean package cache"
/// 3. "Uninstall unused applications"
/// 4. "Remove old kernel versions"
/// 5. "Clean up log files"
/// 0. "Exit"
///
/// # Examples
///
/// ```
/// use multidialogue::DiskSpaceOptimizerItems;
///
/// fn main() {
///     let commands = get_commands();
///
///     for item in commands.items() {
///         println!("{}. {}", item.id(), item.label());
///     }
/// }
/// ```
fn get_commands() -> multidialogue::DiskSpaceOptimizerItems<i32> {
    multidialogue::DiskSpaceOptimizerItems::new()
        .with_option(multidialogue::SelectableItem::new(1, "Remove unnecessary packages"))
        .with_option(multidialogue::SelectableItem::new(2, "Clean package cache"))
        .with_option(multidialogue::SelectableItem::new(3, "Uninstall unused applications"))
        .with_option(multidialogue::SelectableItem::new(4, "Remove old kernel versions"))
        .with_option(multidialogue::SelectableItem::new(5, "Clean up log files"))
        .with_option(multidialogue::SelectableItem::new(0, "Exit"))
}

/// This function takes a command as a string and an array of arguments as string slices, and then
/// executes the command with those arguments. If the command is successful, it returns the output
/// of the command as a string. If the command fails, it returns an error message that includes the
/// exit code and the stderr output.
///
/// # Examples
///
/// ```
/// use std::process::Command;
///
/// use anyhow::{Context, Result};
///
/// fn execute_cmd(cmd: &str, args: &[&str]) -> Result<String> {
///     let cmd_str = format!("{cmd} {args}", cmd = cmd, args = args.join(" "));
///     println!("Executing: {}", cmd_str);
///
///     let output = Command::new(cmd)
///         .args(args)
///         .output()
///         .with_context(|| format!("Failed to execute command: {}", cmd_str))?;
///
///     if output.status.success() {
///         let stdout = String::from_utf8_lossy(&output.stdout);
///         println!("Command output:\n{}", stdout);
///         Ok(stdout.trim().to_string())
///     } else {
///         let stderr = String::from_utf8_lossy(&output.stderr);
///         Err(anyhow!("Command failed with exit code {}: {}", output.status, stderr.trim()))
///     }
/// }
///
/// fn main() -> Result<()> {
///     let output = execute_cmd("git", &["status"])?;
///     println!("{}", output);
///     Ok(())
/// }
/// ```
///
/// This will run the git status command and print its output to the console.
///
/// Note that this function uses the `anyhow` crate to provide a more detailed error message when a
/// command fails. If you don't want to use this crate, you can replace the `Err(anyhow!(...))`
/// line with a regular `Err(...)` line that returns a string error message. Additionally, you need
/// to add the `anyhow` crate to your `Cargo.toml` file as a dependency.
fn execute_cmd(cmd: &str, args: &[&str]) -> Result<String> {
    let cmd_str = format!("{cmd} {args}", cmd = cmd, args = args.join(" "));
    println!("Executing: {cmd_str}", cmd_str = cmd_str,);

    let output = Command::new(cmd)
        .args(args)
        .output()
        .with_context(|| anyhow!(format!("Failed to execute command: {cmd_str}", cmd_str = cmd_str)))?;
    println!("{output:?}");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command output:\n{}", stdout);
        Ok(stdout.trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("Command failed with exit code {}: {}", output.status, stderr.trim()))
    }
}

/// Reads a line of text from standard input and returns it as a `String`.
///
/// # Errors
///
/// Returns an error if flushing standard output fails or if reading from standard input fails.
///
/// # Examples
///
/// ```
/// use std::io;
///
/// fn run() -> io::Result<()> {
///     let input = read_line()?;
///     println!("You entered: {}", input);
///     Ok(())
/// }
/// ```
fn read_line() -> Result<String> {
    let mut buffer = String::new();
    io::stdout().flush().context("Failed to flush stdout")?;
    io::stdin().read_line(&mut buffer).with_context(|| "Failed to read input")?;

    Ok(buffer)
}

/// Reads a file and returns it as a `Vec<String>`.
///
/// # Arguments
///
/// * `filename` - The path to the file to read.
///
/// # Errors
///
/// This function will return an error if the file cannot be read.
#[allow(dead_code)]
fn read_file(filename: &str) -> io::Result<Vec<String>> {
    let file = std::fs::File::open(filename)?;
    let lines = io::BufReader::new(file).lines().collect::<io::Result<Vec<String>>>()?;

    Ok(lines)
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// REGION_END: helper functions
//////////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////////////
// REGION_START: mod multidialogue
//////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) mod multidialogue {
    use std::{cmp, fmt};

    use anyhow::{Error, Result};
    use dialoguer::{theme::ColorfulTheme, MultiSelect};

    /// Runs the dialoguer prompt for selecting one or more options from the list of selectable
    /// items.
    ///
    /// # Arguments
    ///
    /// * `items`: A reference to `DiskSpaceOptimizerItems<T>` containing the list of selectable items.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of references to `SelectableItem<T>` structs that
    /// were selected by the user.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is an issue with the prompt interaction.
    pub(crate) fn run_dialoguer<T>(items: &DiskSpaceOptimizerItems<T>) -> Result<Vec<&SelectableItem<T>>>
    where
        T: fmt::Display + fmt::Debug + cmp::PartialEq<i32> + cmp::Eq + Copy,
    {
        // Set defaults for the prompt.
        let options = &mut items.options.as_ref().unwrap();
        let mut defaults: Vec<bool> = vec![false; options.len()];
        if let Some(last) = defaults.last_mut() {
            let opt = options.last().unwrap();
            debug_assert_eq!(opt.key, 0);
            debug_assert_eq!(opt.text, String::from("Exit"));
            *last = true; // The last command exit will always be selected by default.
        }

        debug_assert_eq!(items.options.as_ref().unwrap().len(), defaults.len());

        // Run the dialoguer prompt.
        let selections: Vec<usize> = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select an option: (space to select, enter to confirm)")
            .items(&format_prompt_item(items).unwrap())
            .defaults(&defaults[..])
            .interact()?;

        // Return the selected items.
        let selections: Vec<&SelectableItem<T>> =
            selections.into_iter().map(|i| (&items.options.as_ref().unwrap().as_slice()[i])).collect();

        Ok(selections)
    }

    /// Formats the prompt items for the dialoguer prompt.
    ///
    /// # Arguments
    ///
    /// * `items`: A reference to `DiskSpaceOptimizerItems<T>` containing the list of selectable items.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of formatted strings representing the selectable
    /// items.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is an issue with formatting the prompt items.
    pub(crate) fn format_prompt_item<T>(items: &DiskSpaceOptimizerItems<T>) -> Result<Vec<String>, Error>
    where
        T: fmt::Display,
    {
        let res = items
            .options
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|item: &SelectableItem<T>| format!("{key}: {text}", key = item.key, text = &item.text[..]))
            .collect();

        Ok(res)
    }

    /// Represents a selectable item in the disk space optimizer. It contains a key of type `T` and
    /// a text description. It can be cloned if the key type implements `Clone`, and can be
    /// created using `SelectableItem::new`.
    #[derive(Debug)]
    pub(crate) struct SelectableItem<T> {
        pub(crate) key: T,
        pub(crate) text: String,
    }

    impl<T: Clone> Clone for SelectableItem<T> {
        fn clone(&self) -> Self {
            SelectableItem { key: self.key.clone(), text: self.text.clone() }
        }
    }

    impl<T: ToString> SelectableItem<T> {
        /// Creates a new `SelectableItem` with the given key and text.
        pub(crate) fn new(key: T, text: &str) -> Self {
            Self { key, text: text.to_string() }
        }
    }

    /// Represents a collection of selectable items in the disk space optimizer.
    ///
    /// It can be iterated over using a for loop, and can be created using
    /// `DiskSpaceOptimizerItems::new`, and then adding options using
    /// `DiskSpaceOptimizerItems::with_option` or `DiskSpaceOptimizerItems::with_options`.
    #[derive(Clone)]
    pub(crate) struct DiskSpaceOptimizerItems<T> {
        pub(crate) options: Option<Vec<SelectableItem<T>>>,
    }

    impl<T: fmt::Debug> fmt::Debug for DiskSpaceOptimizerItems<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "DiskSpaceOptimizerItems {{ options: ")?;
            if let Some(options) = &self.options {
                write!(f, "{:?}", options)?;
            } else {
                write!(f, "None")?;
            }
            write!(f, " }}")
        }
    }

    impl<T> std::iter::IntoIterator for DiskSpaceOptimizerItems<T> {
        type Item = SelectableItem<T>;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.options.unwrap_or_default().into_iter()
        }
    }

    impl<T> DiskSpaceOptimizerItems<T> {
        /// Creates a new `DiskSpaceOptimizerItems` instance with no options.
        pub(crate) fn new() -> Self {
            Self { options: None }
        }

        /// Adds the given option to the list of options.
        pub(crate) fn with_option(mut self, option: SelectableItem<T>) -> Self {
            let mut options = self.options.unwrap_or_default();
            options.push(option);
            self.options = Some(options);
            self
        }

        /// Adds the given list of options to the list of options.
        #[allow(dead_code)]
        pub(crate) fn with_options(mut self, options: Vec<SelectableItem<T>>) -> Self {
            self.options = Some(options);
            self
        }
    }
}
//////////////////////////////////////////////////////////////////////////////////////////////////
// REGION_END: mod multidialogue
//////////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////////////
// REGION_START: mod cli
//////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) mod cli {
    //! This is a Rust module that defines a command-line interface (CLI) tool for optimizing disk
    //! space. The module is called `cli` and is defined with `pub(crate)` visibility, which means
    //! it is visible within the same crate, but not outside of it.
    //!
    //! The module uses several external crates, including `anyhow`, `clap`, and `dialoguer`. The
    //! `Cli` struct is defined using the `Parser` and `clap` macros, and represents the top-level
    //! command of the CLI. It has a single field called `command` that is an optional `Commands`
    //! enum.
    //!
    //! The `Commands` enum represents the subcommands of the CLI, and is defined using the
    //! `Subcommand` macro. It has five variants, each of which represents a specific action that
    //! can be taken to optimize disk space. Some of these variants have associated fields that
    //! provide additional information about the action to be taken.
    //!
    //! The `Commands` enum also has a method called `from_selection` that takes a selection index
    //! and returns an instance of the corresponding `Commands` variant. This method is used to map
    //! user input to a specific subcommand when the user selects an action from a list of options.
    //!
    //! Overall, this module provides a flexible and extensible framework for building a disk space
    //! optimizer tool with a CLI interface.

    use std::{
        io::Write,
        process::{Command, Stdio},
    };

    use anyhow::{anyhow, Context, Error, Result};
    use clap::{Parser, Subcommand};
    use dialoguer::{theme::ColorfulTheme, MultiSelect};

    use super::{execute_cmd, read_line};
    // use std::io::BufRead;

    /// A command-line interface tool for optimizing disk space.
    #[derive(Parser, Debug)]
    #[command(author,version,about,long_about=None)]
    #[clap(name = "Disk Space Optimizer", about = "A CLI tool for optimizing disk space")]
    pub(crate) struct Cli {
        #[command(subcommand)]
        pub(crate) command: Option<Commands>,
    }

    /// The available commands that the tool supports.
    #[derive(Subcommand, Debug, Clone)]
    pub(crate) enum Commands {
        /// Removes a package with the specified name.
        // #[structopt(name = "remove-package", about = "Remove a package")]
        RemovePackage {
            /// The name of the package to remove.
            // #[structopt(name = "PACKAGE_NAME", help = "Name of the package to remove")]
            package_name: String,
        },

        /// Cleans the package cache.
        CleanPackageCache,

        /// Uninstalls unused apps.
        UninstallUnusedApps,

        /// Removes old kernels.
        RemoveOldKernels,

        /// Cleans up log files.
        CleanUpLogFiles,
    }

    impl Commands {
        /// Converts a user selection to a `Commands` enum variant.
        ///
        /// # Arguments
        ///
        /// * `selection` - A `usize` value representing the user's menu selection.
        ///
        /// # Returns
        ///
        /// An `Option<Commands>` representing the variant of the `Commands` enum associated with
        /// the user's menu selection. Returns `None` if the selection is not a valid menu option.
        ///
        /// # Example
        ///
        /// ```
        /// use my_crate::Commands;
        ///
        /// let selection = 1;
        /// let command = Commands::from_selection(selection);
        ///
        /// match command {
        ///     Some(Commands::RemovePackage { package_name }) => {
        ///         println!("Removing package: {}", package_name);
        ///     }
        ///     Some(Commands::CleanPackageCache) => {
        ///         println!("Cleaning package cache...");
        ///     }
        ///     Some(Commands::UninstallUnusedApps) => {
        ///         println!("Uninstalling unused apps...");
        ///     }
        ///     Some(Commands::RemoveOldKernels) => {
        ///         println!("Removing old kernels...");
        ///     }
        ///     Some(Commands::CleanUpLogFiles) => {
        ///         println!("Cleaning up log files...");
        ///     }
        ///     None => {
        ///         println!("Invalid selection.");
        ///     }
        /// }
        /// ```
        pub(crate) fn from_selection(selection: usize) -> Option<Self> {
            match selection {
                1 => Some(Commands::RemovePackage { package_name: String::new() }),
                2 => Some(Commands::CleanPackageCache),
                3 => Some(Commands::UninstallUnusedApps),
                4 => Some(Commands::RemoveOldKernels),
                5 => Some(Commands::CleanUpLogFiles),
                _ => None,
            }
        }

        /// Executes a command based on the selected command variant.
        ///
        /// # Arguments
        ///
        /// * `self` - A `Commands` enum variant to execute.
        ///
        /// # Errors
        ///
        /// Returns an `Error` if any error occurs during command execution.
        ///
        /// # Examples
        ///
        /// ```
        /// use my_crate::Commands;
        /// let command = Commands::CleanPackageCache;
        /// let result = command.execute();
        /// assert!(result.is_ok());
        /// ```
        pub(crate) fn execute(&self) -> Result<(), Error> {
            match self {
                Commands::RemovePackage { package_name } => {
                    if package_name.is_empty() {
                        println!("Select packages to remove (comma-separated), or type 'q' to quit:");
                        let output = Command::new("dnf")
                            .arg("list")
                            .arg("--installed")
                            .output()
                            .with_context(|| "Failed to execute command")?;
                        let lhs_to_pipe = String::from_utf8_lossy(&output.stdout);
                        let mut child = Command::new("awk")
                            .args(["{print $1}"]) // '{print $1}' tells awk to print the first column (in this case, the
                            // package names).
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()
                            .with_context(|| "Failed to execute command")?;
                        let _copying = child.stdin.as_mut().unwrap().write_all(lhs_to_pipe.as_bytes());
                        let output = child.wait_with_output()?;
                        let pkgs_installed = String::from_utf8_lossy(&output.stdout).into_owned();

                        println!("Available packages: ");
                        println!("{}", pkgs_installed);
                        let mut pkgs_installed: Vec<&str> =
                            pkgs_installed.as_str().trim().split('\n').collect::<Vec<_>>();
                        pkgs_installed.push("None");
                        let pkgs_selected: Vec<usize> = MultiSelect::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select installed packages to remove:")
                            .items(pkgs_installed.as_slice())
                            .interact()?;

                        let pkgs_selected: Vec<&str> =
                            pkgs_selected.into_iter().map(|value: usize| pkgs_installed[value]).collect();
                        let pkgs = if pkgs_selected.is_empty() {
                            println!("No packages selected. Please try again.");
                            Err(anyhow!("No packages were selected"))
                        } else if pkgs_selected.contains(&"None") {
                            Err(anyhow!("No packages were selected"))
                        } else {
                            Ok(pkgs_selected) // Ok(pkgs_selected.as_slice().join(" "))
                        };

                        if let Ok(pkgs) = pkgs {
                            println!("These packages will be removed:");
                            println!("{:?}", pkgs);
                            let n_pkgs = pkgs.len();
                            println!("Proceed to delete a total of {} package(s): (y/N)", n_pkgs);

                            let mut resp = String::new();
                            std::io::stdin().read_line(&mut resp)?;

                            if resp.trim() != "y" && resp.trim() != "n" {
                                return Err(anyhow!("Invalid response. Type either 'y' or 'n"));
                            }
                            if !(resp.trim() == "y") {
                                return Err(anyhow!("Aborted deleting selected packages."));
                            }

                            println!("Preparing command to remove each of the selected packages");

                            for pkg in &pkgs {
                                let cmd = "sudo";
                                let args = ["dnf", "remove", pkg];
                                let cmd_str = format!("{cmd} {args}", cmd = cmd, args = args.join(" "));
                                println!("Executing: {cmd_str}", cmd_str = cmd_str,);

                                let output = Command::new("sudo")
                                    .args(["dnf", "info", pkg.trim()])
                                    .output()
                                    .with_context(|| {
                                        anyhow!(format!(
                                            "Failed to execute command for package: {package}",
                                            package = pkg
                                        ))
                                    })?;

                                if output.status.success() {
                                    let stdout = String::from_utf8_lossy(&output.stdout);
                                    println!("Command output:\n{}", stdout);
                                } else {
                                    let stderr = String::from_utf8_lossy(&output.stderr);
                                    println!("Command output:\n{}", stderr);

                                    // Uncomment the line below if you want to return an error when
                                    // the command fails. return
                                    // Err(anyhow!( "Command failed with exit code {}: {}",
                                    // output.status, stderr.trim()));
                                }
                            }
                        }
                    } else {
                        println!("Enter package name to remove:");
                        let package_name = read_line()?;
                        execute_cmd("sudo", &["dnf", "remove", package_name.trim()])?;
                    }
                }
                Commands::CleanPackageCache => {
                    execute_cmd("sudo", &["dnf", "clean", "all"])?;
                }
                Commands::UninstallUnusedApps => {
                    execute_cmd("sudo", &["dnf", "autoremove"])?;
                }
                Commands::RemoveOldKernels => {
                    println!("Select kernels to remove (comma-separated), or type 'q' to quit:");
                    let output = Command::new("rpm")
                        .arg("-q")
                        .arg("kernel")
                        .output()
                        .with_context(|| "Failed to execute command")?;
                    println!("Available: {}", String::from_utf8_lossy(&output.stdout));

                    let kernels = String::from_utf8_lossy(&output.stdout).into_owned();
                    let mut kernels: Vec<&str> = kernels.as_str().trim().split('\n').collect::<Vec<_>>();
                    kernels.push("None");

                    let selected_kernels: Vec<usize> = MultiSelect::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter kernel versions to remove:")
                        .items(kernels.as_slice())
                        .interact()?;

                    let selected_kernels: Vec<&str> =
                        selected_kernels.into_iter().map(|value: usize| kernels[value]).collect();

                    let kernels = if selected_kernels.is_empty() {
                        println!("No kernels selected. Please try again.");
                        Err(anyhow!("No kernels were selected")) // std::process::exit(1);
                    } else if selected_kernels.contains(&"None") {
                        Err(anyhow!("No kernels were selected")) // std::process::exit(1);
                    } else {
                        Ok(selected_kernels.as_slice().join(" "))
                    };

                    if let Ok(kernel) = kernels {
                        println!("Copying command to remove kernels: {}", kernel);

                        let mut args = vec!["sudo", "dnf", "remove"];
                        args.push(kernel.as_str());
                        let the_string = args.join(" ");

                        let mut child = Command::new("xsel")
                            .args(["-ib"])
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()
                            .with_context(|| "Failed to execute command")?;

                        let _copying = child.stdin.as_mut().unwrap().write_all(the_string.as_bytes());
                        let output = child.wait_with_output()?;
                        println!("{}", String::from_utf8(output.stdout).unwrap());
                        println!("Paste the command: \"{}\"", the_string);
                    }
                }
                Commands::CleanUpLogFiles => {
                    println!("Enter vacuum time (Default: 7) as days:");
                    let vacuum_time = read_line()?.trim().parse::<u32>().unwrap_or(7);
                    let vacuum_time = format!("--vacuum-time={days}d", days = vacuum_time);
                    execute_cmd("sudo", &["journalctl", vacuum_time.as_str()])?;
                }
            }
            Ok(())
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// REGION_END: mod cli
//////////////////////////////////////////////////////////////////////////////////////////////////
