use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use toml::Table;

use crate::{error::Error, utils::is_initialized};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub ai: AIConfig,
    pub prompt: Table,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AIConfig {
    #[serde(rename = "openai")]
    OpenAI(OpenAIConfig),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct OpenAIConfig {
    pub model: String,
}

pub fn load_config(cwd: &str) -> Result<(Config, String), Error> {
    if !is_initialized(cwd) {
        return Err(Error::AlreadyInitialized);
    }

    let config_str =
        fs::read_to_string(Path::new(&format!("{}/.gitgud/config.toml", cwd))).unwrap();

    let config: Config = toml::from_str(&config_str).unwrap();

    let prompt = fs::read_to_string(Path::new(&format!("{}/.gitgud/prompt.txt", cwd))).unwrap();

    Ok((config, prompt))
}
