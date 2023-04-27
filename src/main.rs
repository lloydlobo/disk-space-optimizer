use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::env::consts::OS;
use std::io::{self, prelude::*};
use std::process::Command;

// #![deny(missing_docs)]

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

const ELLIPSIS: &str = "…";

fn get_commands() -> multidialogue::DiskSpaceOptimizerItems<i32> {
    multidialogue::DiskSpaceOptimizerItems::new()
        .with_option(multidialogue::SelectableItem::new(
            1,
            "Remove unnecessary packages",
        ))
        .with_option(multidialogue::SelectableItem::new(2, "Clean package cache"))
        .with_option(multidialogue::SelectableItem::new(
            3,
            "Uninstall unused applications",
        ))
        .with_option(multidialogue::SelectableItem::new(
            4,
            "Remove old kernel versions",
        ))
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
///         Err(anyhow!(
///             "Command failed with exit code {}: {}",
///             output.status,
///             stderr.trim()
///         ))
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
///
fn execute_cmd(cmd: &str, args: &[&str]) -> Result<String> {
    let cmd_str = format!("{cmd} {args}", cmd = cmd, args = args.join(" "));
    println!("Executing: {cmd_str}", cmd_str = cmd_str,);

    let output = Command::new(cmd).args(args).output().with_context(|| {
        anyhow!(format!(
            "Failed to execute command: {cmd_str}",
            cmd_str = cmd_str
        ))
    })?;
    println!("{output:?}");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command output:\n{}", stdout);
        Ok(stdout.trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!(
            "Command failed with exit code {}: {}",
            output.status,
            stderr.trim()
        ))
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
/// fn main() -> io::Result<()> {
///     let input = read_line()?;
///     println!("You entered: {}", input);
///     Ok(())
/// }
/// ```
fn read_line() -> Result<String> {
    let mut buffer = String::new();
    io::stdout().flush().context("Failed to flush stdout")?;
    io::stdin()
        .read_line(&mut buffer)
        .with_context(|| "Failed to read input")?;

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
///
fn read_file(filename: &str) -> io::Result<Vec<String>> {
    let file = std::fs::File::open(filename)?;
    let lines = io::BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()?;

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use std::{
        fs::File,
        io::{self, Write},
    };
    use tempfile::NamedTempFile;

    #[test]
    fn it_works() {
        let result = 3 + 1;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_tempfile_dependecy() -> io::Result<()> {
        let mut input_file = NamedTempFile::new()?;
        writeln!(input_file, "Hello, world!")?;

        let got = input_file.path().to_str().unwrap();
        assert_str_eq!(&got[0..=8], "/tmp/.tmp");

        let mut file = File::open(got)?;
        let mut output_file = NamedTempFile::new()?;
        io::copy(&mut file, &mut output_file)?;

        let _got = output_file.path().to_str().unwrap();
        let mut output_file = File::open(output_file.path())?;
        let mut buffer = String::new();
        output_file.read_to_string(&mut buffer)?;
        assert_eq!(buffer, "Hello, world!\n");

        Ok(())
    }

    #[test]
    fn test_execute_cmd() -> Result<()> {
        let output = execute_cmd("echo", &["hello", "world"])?;
        assert_eq!(output, "hello world");

        let output = execute_cmd("git", &["status"])?;
        assert!(output.len() > 0);
        Ok(())
    }

    // NOTE: manual intervention required. Type `hello, world` for test to pass.
    //
    // #[test]
    // fn test_read_line() -> io::Result<()> {
    //     let mut input_file = NamedTempFile::new()?;
    //     writeln!(input_file, "hello, world")?;
    //     let mut saved_stdin = std::io::stdin();
    //     let result = read_line()?;
    //     Ok(())
    // }
}

pub(crate) mod multidialogue {
    use std::{cmp, fmt};

    use anyhow::{Error, Result};
    use dialoguer::{theme::ColorfulTheme, MultiSelect};

    /// Runs the dialoguer prompt for selecting one or more options from the list of selectable items.
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
    ///
    pub(crate) fn run_dialoguer<T>(
        items: &DiskSpaceOptimizerItems<T>,
    ) -> Result<Vec<&SelectableItem<T>>>
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
        let selections: Vec<&SelectableItem<T>> = selections
            .into_iter()
            .map(|i| (&items.options.as_ref().unwrap().as_slice()[i]))
            .collect();

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
    /// Returns a `Result` containing a vector of formatted strings representing the selectable items.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is an issue with formatting the prompt items.
    ///
    pub(crate) fn format_prompt_item<T>(
        items: &DiskSpaceOptimizerItems<T>,
    ) -> Result<Vec<String>, Error>
    where
        T: fmt::Display,
    {
        let res = items
            .options
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|item: &SelectableItem<T>| {
                format!("{key}: {text}", key = item.key, text = &item.text[..])
            })
            .collect();

        Ok(res)
    }

    /// Represents a selectable item in the disk space optimizer. It contains a key of type `T` and a
    /// text description. It can be cloned if the key type implements `Clone`, and can be created using
    /// `SelectableItem::new`.
    #[derive(Debug)]
    pub(crate) struct SelectableItem<T> {
        pub(crate) key: T,
        pub(crate) text: String,
    }

    impl<T: Clone> Clone for SelectableItem<T> {
        fn clone(&self) -> Self {
            SelectableItem {
                key: self.key.clone(),
                text: self.text.clone(),
            }
        }
    }

    impl<T: ToString> SelectableItem<T> {
        /// Creates a new `SelectableItem` with the given key and text.
        pub(crate) fn new(key: T, text: &str) -> Self {
            Self {
                key,
                text: text.to_string(),
            }
        }
    }

    /// Represents a collection of selectable items in the disk space optimizer.
    ///
    /// It can be iterated over using a for loop, and can be created using
    /// `DiskSpaceOptimizerItems::new`, and then adding options using
    /// `DiskSpaceOptimizerItems::with_option` or `DiskSpaceOptimizerItems::with_options`.
    ///
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
        pub(crate) fn with_options(mut self, options: Vec<SelectableItem<T>>) -> Self {
            self.options = Some(options);
            self
        }
    }
}

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

    use super::{execute_cmd, read_line};
    use anyhow::{anyhow, Context, Error, Result};

    use clap::{Parser, Subcommand};
    use dialoguer::{theme::ColorfulTheme, MultiSelect};
    use std::{
        io::{BufRead, Write},
        process::{Command, Stdio},
    };

    /// A command-line interface tool for optimizing disk space.
    #[derive(Parser, Debug)]
    #[command(author,version,about,long_about=None)]
    #[clap(
        name = "Disk Space Optimizer",
        about = "A CLI tool for optimizing disk space"
    )]
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
        ///     },
        ///     Some(Commands::CleanPackageCache) => {
        ///         println!("Cleaning package cache...");
        ///     },
        ///     Some(Commands::UninstallUnusedApps) => {
        ///         println!("Uninstalling unused apps...");
        ///     },
        ///     Some(Commands::RemoveOldKernels) => {
        ///         println!("Removing old kernels...");
        ///     },
        ///     Some(Commands::CleanUpLogFiles) => {
        ///         println!("Cleaning up log files...");
        ///     },
        ///     None => {
        ///         println!("Invalid selection.");
        ///     },
        /// }
        /// ```
        pub(crate) fn from_selection(selection: usize) -> Option<Self> {
            match selection {
                1 => Some(Commands::RemovePackage {
                    package_name: String::new(),
                }),
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
                        execute_cmd("sudo", &["dnf", "remove", package_name.trim()])?;
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
                    let kernels: Vec<&str> =
                        kernels.as_str().trim().split('\n').collect::<Vec<_>>();

                    let selected_kernels: Vec<usize> =
                        MultiSelect::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter kernel versions to remove:")
                            .items(kernels.as_slice())
                            .interact()?;

                    let selected_kernels: Vec<&str> = selected_kernels
                        .into_iter()
                        .map(|value: usize| kernels[value])
                        .collect();

                    let kernels = if selected_kernels.is_empty() {
                        println!("No kernels selected. Please try again.");
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
                            .args(&["-ib"])
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()
                            .with_context(|| "Failed to execute command")?;

                        let _copying = child
                            .stdin
                            .as_mut()
                            .unwrap()
                            .write_all(the_string.as_bytes());
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

// clipboard.set_text(the_string.clone()).unwrap();
// let readline_confirm = &read_line().unwrap_or(String::from("n"));
// let readline_confirm = readline_confirm.trim().to_lowercase();
// if readline_confirm == "y" || readline_confirm == "yes" {
//     // execute_cmd("sudo", &["dnf", "remove", kernel.as_str()])?;
//     let output_file = "/tmp/dnf_output.txt"; // path to output file.
//     let output = Command::new("sudo")
//         .args(["dnf", "remove", kernel.as_str()])
//         .stdout(Stdio::piped())
//         .stderr(Stdio::inherit())
//         .spawn()?
//         .wait_with_output()
//         .with_context(|| "Failed to execute command")?; // let output = Command::new("sudo") .args(["dnf", "remove", kernel.as_str()]) .stdout(Stdio::piped()) .spawn()? .wait_with_output() .with_context(|| "Failed to execute command")?;
//
//     // Write the output to a file.
//     let mut file = std::fs::File::create(output_file)?;
//     file.write_all(&output.stdout)?;
//
//     // read and display the contents of the file to the user.
//     let file = std::fs::File::open(output_file)?;
//     let reader = BufReader::new(file);
//     for line in reader.lines() {
//         println!("{line}", line = line?);
//     }
//     // io::stdout().write_all(&output.stdout)?;
// } else {
//     std::process::exit(1)
// }
