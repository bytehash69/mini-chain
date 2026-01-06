use std::{process::exit, str::FromStr};

use console::style;
use inquire::{InquireError, Select, Text};

use crate::commands::CommandGroup;

pub fn prompt() -> anyhow::Result<CommandGroup> {
    let cmd = Select::new(
        "Choose a command to continue:",
        vec![
            CommandGroup::NewTransaction,
            CommandGroup::MineBlock,
            CommandGroup::ChangeDifficulty,
            CommandGroup::ChangeReward,
            CommandGroup::Exit,
        ],
    )
    .prompt()?;

    if matches!(cmd, CommandGroup::Exit) {
        println!("{}", style("Goodbye 👋").bold().yellow());
        exit(0);
    }

    Ok(cmd)
}

pub fn prompt_input_data<T>(msg: &str) -> T
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    loop {
        let input = match Text::new(msg).prompt() {
            Ok(v) => v,
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                println!("{}", style("Operation interrupted. Exiting.").bold().red());
                exit(0);
            }
            Err(e) => {
                println!("Invalid input: {}. Try again.", e);
                continue;
            }
        };

        match input.parse::<T>() {
            Ok(value) => return value,
            Err(e) => println!("Parse error: {}. Try again.", e),
        }
    }
}
