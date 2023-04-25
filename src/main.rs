use anyhow::{Context, Ok};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::process::Command;
// use std::intrinsics::mir::Return;

const ELLIPSIS: &str = "…";

fn main() {
    let user_os: &str = "Fedora";
    println!(
        "Welcome to the {user_os} disk space optimizer CLI!",
        user_os = user_os
    );

    let commands = get_commands();
    for item in commands {
        println!("{}: {}", item.key, item.text);
    }

    let selections = run_dialoguer(&commands).unwrap();

    run_commands(selections).unwrap();

    // let args = Args::parse();
    // for _ in 0..args.count { println!("Hello, {}!", args.name); }
}

fn run_commands(selections: Vec<SelectableItem<i32>>) -> anyhow::Result<()> {
    for selection in selections {
        println!(
            "[{key}]: Running {command}{ELLIPSIS}",
            key = selection.key,
            command = selection.text
        );
        match selection.key {
            0 => {
                assert!(selection.text.to_string() == "");
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
                // let output = Command::new("sudo")
                //     .arg("journalctl")
                //     .arg("--vacuum-time=7d")
                //     .output()
                //     .expect("Failed to execute command");
                //
                // println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
    Ok(())
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

#[derive(Clone, Debug)]
struct DiskSpaceOptimizerItems<T> {
    options: Option<Vec<SelectableItem<T>>>,
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

    fn with_options(mut self, options: Vec<SelectableItem<T>>) -> Self {
        self.options = Some(options);
        self
    }
}

fn get_commands() -> DiskSpaceOptimizerItems<i32> {
    DiskSpaceOptimizerItems::new().with_options(vec![
        SelectableItem::new(1, "Remove unnecessary packages"),
        SelectableItem::new(2, "Clean package cache"),
        SelectableItem::new(3, "Uninstall unused applications"),
        SelectableItem::new(4, "Remove old kernel versions"),
        SelectableItem::new(5, "Clean up log files"),
        SelectableItem::new(0, "Exit"),
    ])
}

fn run_dialoguer<T>(items: &DiskSpaceOptimizerItems<T>) -> anyhow::Result<Vec<SelectableItem<T>>> {
    let defaults = &[false, false, false, false, false, true];
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select an option:")
        .items(
            &items
                .options
                .clone()
                .unwrap()
                .iter()
                .map(|item| format!("{key}: {text}", key = &item.key, text = &item.text[..]))
                .collect::<Vec<_>>(),
        )
        .defaults(&defaults[..])
        .interact()?;

    let selections: Vec<SelectableItem<T>> = selections
        .iter()
        .map(|index| items.options.clone().unwrap()[*index].clone())
        .collect();

    Ok(selections)
}

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
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
