use super::*;
use anyhow::Result;
use pretty_assertions::{assert_eq, assert_str_eq};
use std::{
    fs::File,
    io::{self, Write},
};
use tempfile::NamedTempFile;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

// trait Command { fn execute(&self) -> Result<()>; }
// struct MockCommand;
// impl Command for MockCommand {
//     fn execute(&self) -> Result<()> { println!("Mock command executed"); Ok(()) }
// }

fn main_with_cli(cli: cli::Cli) -> Result<()> {
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

#[test]

fn mytest() {
    let mut mock = MockMyTrait::new();
    mock.expect_foo().with(eq(4)).times(1).returning(|x| x + 1);
    assert_eq!(5, mock.foo(4));
}

#[test]
fn test_main() {
    // Test with command argument. It should not use dialoguer.
    let cli_with_command = cli::Cli {
        command: Some(cli::Commands::RemoveOldKernels), // command: Some(Box::new(MockCommand {})),
    };
    assert!(main_with_cli(cli_with_command).is_ok());

    // Test without command argument. It should use dialoguer.
    let cli_without_command = cli::Cli { command: None };
    assert!(main_with_cli(cli_without_command).is_ok());
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
