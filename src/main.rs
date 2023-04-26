use anyhow::{anyhow, Context, Error, Result};
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::io::{self, Write};
use std::process::{Command, Stdio}; // use std::intrinsics::mir::Return;

const ELLIPSIS: &str = "…";

fn main() {
    let cli = Cli::parse();

    let user_os: &str = "Fedora";
    println!(
        "Welcome to the {user_os} disk space optimizer CLI!",
        user_os = user_os
    );
    if let Some(command) = &cli.command {
        command.execute().unwrap();
        // Commands::execute(command).unwrap();
    } else {
        let commands: DiskSpaceOptimizerItems<i32> = get_commands();
        let selections: Vec<&SelectableItem<i32>> = run_dialoguer(&commands).unwrap();
        selections.into_iter().for_each(|selection| {
            if let Some(command) = Commands::from_selection(selection.key as usize) {
                if let Err(err) = command.execute() {
                    println!("Error: {err}", err = anyhow!(err));
                }
            }
        });
    }
}

/// This function takes a command as a string and an array of arguments as string slices, and then executes the command with those arguments. If the command is successful, it returns the output of the command as a string. If the command fails, it returns an error message that includes the exit code and the stderr output.
///
/// Use this function in your programs to execute shell commands and capture their output. For example, you can use it to run a Git command and capture the output:
///
/// ```rust
/// fn main() -> Result<()> {
///     let output = execute_command("git", &["status"])?;
///     println!("{}", output);
///     Ok(())
/// }
/// ```
///
/// This will run the git status command and print its output to the console.
///
/// NOTE: that this function uses the anyhow crate to provide a more detailed error message when a command fails. If you don't want to use this crate, you can replace the Err(anyhow!(...)) line with a regular Err(...) line that returns a string error message.
fn execute_cmd(cmd: &str, args: &[&str]) -> Result<String> {
    let cmd_str = format!("{cmd} {args}", cmd = cmd, args = args.join(" "));
    println!(
        "Executing command: {cmd_str}{ELLIPSIS}",
        cmd_str = cmd_str,
        ELLIPSIS = ELLIPSIS
    );

    let output = Command::new(cmd)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute command: {cmd_str}", cmd_str = cmd_str))?;

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

// #[derive(Parser)]
// #[command(setting = AppSettings::ColoredHelp)]
#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// #[structopt(name = "remove-package", about = "Remove a package")]
    RemovePackage {
        /// #[structopt(name = "PACKAGE_NAME", help = "Name of the package to remove")]
        package_name: String,
    },
    CleanPackageCache,
    UninstallUnusedApps,
    RemoveOldKernels,
    CleanUpLogFiles,
}

impl Commands {
    fn from_selection(selection: usize) -> Option<Self> {
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

    fn execute(&self) -> Result<(), Error> {
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
            #[rustfmt::skip]
            Commands::RemoveOldKernels => {
                println!("Select kernels to remove (comma-separated), or type 'q' to quit:");
                let output = Command::new("rpm") .arg("-q") .arg("kernel") .output() .with_context(|| "Failed to execute command")?;
                println!("Available: {}", String::from_utf8_lossy(&output.stdout));

                let kernels = String::from_utf8_lossy(&output.stdout).into_owned();
                let kernels: Vec<&str> = kernels.as_str().trim().split('\n').collect::<Vec<_>>();

                let selections: Vec<usize> = MultiSelect::with_theme(&ColorfulTheme::default()) .with_prompt("Enter kernel versions to remove:") .items(kernels.as_slice()) .interact()?;
                let selected_kernels: Vec<&str> = selections .into_iter() .map(|value: usize| kernels[value]) .collect();

                let kernels = if selected_kernels.is_empty() {
                    println!("No kernels selected. Please try again.");
                    Err(anyhow!("No kernels were selected")) // std::process::exit(1);
                } else {
                    Ok(selected_kernels.as_slice().join(" "))
                };

                match kernels {
                    Ok(kernel) => {
                        println!("Removing kernels: {}\nAre you sure?(Y/n)", kernel);
                        if let "yes" | "y" = read_line().with_context(|| anyhow!("Error while reading line"))?.as_str().to_lowercase().as_str() {
                            let output = Command::new("sudo").args(["dnf", "remove", kernel.as_str()]).stdout(Stdio::piped()).spawn()?
                                .wait_with_output()
                                .with_context(|| "Failed to execute command")?;

                            io::stdout().write_all(&output.stdout)?;
                        } else {
                            std::process::exit(1)
                        }
                    }
                    Err(e) => unreachable!( "{}", format!("called `Result::unwrap()` on an `Err` value: {e}")),
                };
            }
            Commands::CleanUpLogFiles => {
                println!("Enter vacuum time (Default: 7) as days:");
                let vacuum_time = read_line()?.trim().parse::<u32>().unwrap_or(7);
                execute_cmd(
                    "sudo",
                    &[
                        "journalctl",
                        format!("--vacuum-time={days}d", days = vacuum_time).as_str(),
                    ],
                )?;
            }
        }
        Ok(())
    }
}

fn read_line() -> Result<String> {
    let mut buffer = String::new();
    io::stdout().flush().context("Failed to flush stdout")?;
    io::stdin()
        .read_line(&mut buffer)
        .with_context(|| "Failed to read input")?;
    Ok(buffer)
}

#[derive(Debug)]
struct SelectableItem<T> {
    key: T,
    text: String,
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
    fn new(key: T, text: &str) -> Self {
        Self {
            key,
            text: text.to_string(),
        }
    }
}

#[derive(Clone)]
struct DiskSpaceOptimizerItems<T> {
    options: Option<Vec<SelectableItem<T>>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for DiskSpaceOptimizerItems<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn new() -> Self {
        Self { options: None }
    }

    fn with_option(mut self, option: SelectableItem<T>) -> Self {
        let mut options = self.options.unwrap_or_default();
        options.push(option);
        self.options = Some(options);
        self
    }

    fn with_options(mut self, options: Vec<SelectableItem<T>>) -> Self {
        self.options = Some(options);
        self
    }
}

fn get_commands() -> DiskSpaceOptimizerItems<i32> {
    DiskSpaceOptimizerItems::new()
        .with_option(SelectableItem::new(1, "Remove unnecessary packages"))
        .with_option(SelectableItem::new(2, "Clean package cache"))
        .with_option(SelectableItem::new(3, "Uninstall unused applications"))
        .with_option(SelectableItem::new(4, "Remove old kernel versions"))
        .with_option(SelectableItem::new(5, "Clean up log files"))
        .with_option(SelectableItem::new(0, "Exit"))
}

fn run_dialoguer<T>(items: &DiskSpaceOptimizerItems<T>) -> Result<Vec<&SelectableItem<T>>>
where
    T: std::fmt::Display,
{
    let defaults = &[false, false, false, false, false, true];

    debug_assert_eq!(items.options.as_ref().unwrap().len(), defaults.len());

    let selections: Vec<usize> = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select an option:")
        .items(&format_prompt_item(items).unwrap())
        .defaults(&defaults[..])
        .interact()?;

    let selections: Vec<&SelectableItem<T>> = selections
        .into_iter()
        .map(|i| (&items.options.as_ref().unwrap().as_slice()[i]))
        .collect();

    Ok(selections)
}

fn format_prompt_item<T>(items: &DiskSpaceOptimizerItems<T>) -> Result<Vec<String>, Error>
where
    T: std::fmt::Display,
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

fn run_main() {
    println!("Welcome to the Fedora disk space optimizer CLI!");

    loop {
        println!("Please select an option:");
        println!("1. Remove unnecessary packages");
        println!("2. Clean package cache");
        println!("3. Uninstall unused applications");
        println!("4. Remove old kernel versions");
        println!("5. Clean up log files");
        println!("0. Exit");

        let mut option = String::new();
        std::io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input");

        match option.trim() {
            "1" => {
                println!("Enter package name to remove:");
                let mut package_name = String::new();
                std::io::stdin()
                    .read_line(&mut package_name)
                    .expect("Failed to read input");

                let output = Command::new("sudo")
                    .arg("dnf")
                    .arg("remove")
                    .arg(package_name.trim())
                    .output()
                    .expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "2" => {
                let output = Command::new("sudo")
                    .arg("dnf")
                    .arg("clean")
                    .arg("all")
                    .output()
                    .expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "3" => {
                println!("Please use GNOME Software or command line to uninstall applications.");
            }
            "4" => {
                let output = Command::new("rpm")
                    .arg("-q")
                    .arg("kernel")
                    .output()
                    .expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&output.stdout));

                println!("Enter kernel version to remove:");
                let mut kernel_version = String::new();
                std::io::stdin()
                    .read_line(&mut kernel_version)
                    .expect("Failed to read input");

                let output = Command::new("sudo")
                    .arg("dnf")
                    .arg("remove")
                    .arg(kernel_version.trim())
                    .output()
                    .expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "5" => {
                let output = Command::new("sudo")
                    .arg("journalctl")
                    .arg("--vacuum-time=7d")
                    .output()
                    .expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "0" => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}

fn run_commands(selections: Vec<&SelectableItem<i32>>) -> Result<()> {
    for selection in selections {
        if selection.key != 0 {
            println!(
                "[{key}]: Running {command}{ELLIPSIS}",
                key = selection.key,
                command = selection.text
            );
        }
        match selection.key {
            0 => {
                assert!(selection.key == 0);
                println!("[{key}]: Exiting{ELLIPSIS}", key = selection.key);
                break;
            }
            1 => {
                assert!(selection.key == 1);
                // TODO: list available package name.
                println!("Enter package name to remove");
                let mut package_name_readline = String::new();
                std::io::stdin()
                    .read_line(&mut package_name_readline)
                    .context("Failed to read input")?;
                let output = Command::new("sudo")
                    .arg("dnf")
                    .arg("remove")
                    .arg(package_name_readline.trim())
                    .output()
                    .context("Failed to execute command")?;
                println!("{result}", result = String::from_utf8_lossy(&output.stdout));
            }
            5 => {
                assert!(selection.key == 5);
                println!("Enter vacuum time (Default: 7) as days");
                let vacuum_time = 7;
                let mut vacuum_time_readline = String::new();
                std::io::stdin()
                    .read_line(&mut vacuum_time_readline)
                    .context("Failed to read input")
                    .unwrap_or(vacuum_time);
                let output = Command::new("sudo")
                    .arg("journalctl")
                    .arg(format!("--vacuum-time={days}", days = vacuum_time_readline))
                    .output()
                    .context("Failed to execute command")?;
                println!("{result}", result = String::from_utf8_lossy(&output.stdout));
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
    Ok(())
}
