use std::{collections::HashMap, env::current_dir, fs};

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{error::Error, utils};

#[derive(Debug, Parser)]
#[command(about = "Initialize GitGud")]
pub struct Command {
    #[arg(long)]
    pub cwd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Context {}

pub fn exec(cmd: &Command) -> Result<(), Error> {
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

    if utils::is_initialized(&git_root) {
        return Err(Error::AlreadyInitialized);
    }

    let stubs = load_stubs();

    let convention = "conventional";

    fs::create_dir(format!("{}/.gitgud", git_root)).unwrap();

    fs::write(
        format!("{}/.gitgud/config.toml", git_root),
        stubs
            .get(format!("{}/config", convention).as_str())
            .unwrap(),
    )
    .unwrap();

    fs::write(
        format!("{}/.gitgud/prompt.txt", git_root),
        stubs
            .get(format!("{}/prompt", convention).as_str())
            .unwrap(),
    )
    .unwrap();

    fs::write(
        format!("{}/.gitgud/README.md", git_root),
        stubs.get("readme").unwrap(),
    )
    .unwrap();

    println!(
        r#"

░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░██████╗░██╗████████╗░░░██████╗░██╗░░░██╗██████╗░░
░██╔════╝░██║╚══██╔══╝░░██╔════╝░██║░░░██║██╔══██╗░
░██║░░██╗░██║░░░██║░░░░░██║░░██╗░██║░░░██║██║░░██║░
░██║░░╚██╗██║░░░██║░░░░░██║░░╚██╗██║░░░██║██║░░██║░
░╚██████╔╝██║░░░██║░░░░░╚██████╔╝╚██████╔╝██████╔╝░
░░╚═════╝░╚═╝░░░╚═╝░░░░░░╚═════╝░░╚═════╝░╚═════╝░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

GitGud has been initialized!

"#
    );

    Ok(())
}

fn load_stubs() -> HashMap<&'static str, &'static str> {
    let mut stubs = HashMap::new();

    stubs.insert("readme", include_str!("../../stubs/README.md"));

    stubs.insert(
        "conventional/config",
        include_str!("../../stubs/conventional/config.toml"),
    );

    stubs.insert(
        "conventional/prompt",
        include_str!("../../stubs/conventional/prompt.txt"),
    );

    stubs
}
