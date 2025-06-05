pub mod commit;
pub mod init;

use std::env::current_dir;

use clap::{Parser, Subcommand};
use inquire::Confirm;

use crate::{error::Error, utils};

#[derive(Debug, Parser)]
#[command(name = "GitGud")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init(init::Command),
    Commit(commit::Command),
}

pub async fn exec() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Init(cmd)) => {
            init::exec(cmd)?;
        }
        Some(Command::Commit(cmd)) => {
            commit::exec(cmd).await?;
        }
        None => {
            let cwd = current_dir().unwrap().display().to_string();

            let git_root = utils::find_git_root(&cwd)
                .ok_or(Error::NotGitRepository)?
                .display()
                .to_string();

            match utils::is_initialized(&git_root) {
                true => {
                    commit::exec(&commit::Command {
                        cwd: Some(git_root),
                    })
                    .await?;
                }
                false => {
                    if let Ok(true) =
                        Confirm::new("GitGut is not initialized, do you want to run `gitgut init`?")
                            .prompt()
                    {
                        init::exec(&init::Command {
                            cwd: Some(git_root),
                        })?;
                    }
                }
            }
        }
    };

    Ok(())
}
