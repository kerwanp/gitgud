use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot find git repository")]
    NotGitRepository,
    #[error("GitGud is already initialized in this repository")]
    AlreadyInitialized,
    #[error("GitGud is not initialized: run `gitgud init` to get started")]
    NotInitialized,

    #[error("failed spawning git")]
    FailedSpawningGit(#[from] io::Error),

    #[error("The environment variable `{0}` is missing")]
    MissingVariable(String),
}
