use std::{env::current_dir, os::unix::process::CommandExt, process};

use clap::Parser;

use crate::{config, error::Error, prompt, utils};

#[derive(Debug, Parser)]
#[command(about = "Commit staged files with generated message")]
pub struct Command {
    #[arg(long)]
    pub cwd: Option<String>,
}

pub async fn exec(cmd: &Command) -> Result<(), Error> {
    let cwd = cmd
        .cwd
        .clone()
        .unwrap_or(current_dir().unwrap().display().to_string());

    let git_root = match utils::find_git_root(&cwd) {
        Some(root) => root,
        None => {
            return Err(Error::NotGitRepository);
        }
    }
    .display()
    .to_string();

    let (config, prompt) = config::load_config(&git_root)?;

    let commit_message = prompt::generate_commit(config, &prompt, &git_root).await?;

    let err = process::Command::new("git")
        .arg("commit")
        .arg("--edit")
        .arg("-m")
        .arg(commit_message)
        .exec();

    Err(Error::FailedSpawningGit(err))
}
