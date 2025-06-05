use std::{
    env,
    process::{Command, exit},
};

use openai::{
    Credentials,
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
};
use spinners::Spinner;
use tinytemplate::TinyTemplate;

use crate::{
    config::{AIConfig, Config, OpenAIConfig},
    error::Error,
};

pub async fn generate_commit(config: Config, prompt: &str, cwd: &str) -> Result<String, Error> {
    let mut sp = Spinner::new(
        spinners::Spinners::Aesthetic,
        "Generating your commit message...".to_string(),
    );

    let mut tt = TinyTemplate::new();
    tt.add_template("prompt", prompt).unwrap();

    let output = Command::new("git")
        .current_dir(cwd)
        .arg("diff")
        .arg("--cached")
        .output()
        .unwrap();

    if !output.status.success() {
        eprintln!("Git diff command failed");
        exit(1)
    }

    let diff_text = String::from_utf8_lossy(&output.stdout).to_string();

    let mut context = config.prompt;

    context.insert("git_diff".to_string(), toml::Value::String(diff_text));

    let prompt = tt.render("prompt", &context).unwrap();

    let message = match config.ai {
        AIConfig::OpenAI(OpenAIConfig { model }) => prompt_openai(&model, prompt).await?,
    };

    sp.stop();

    Ok(message)
}

async fn prompt_openai(model: &str, prompt: String) -> Result<String, Error> {
    let api_key =
        env::var("OPENAI_KEY").map_err(|_| Error::MissingVariable("OPENAI_KEY".to_string()))?;

    let credentials = Credentials::new(api_key, "https://api.openai.com/v1".to_string());

    let messages = [ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(prompt),
        ..Default::default()
    }];

    let chat_completion = ChatCompletion::builder(model, messages.clone())
        .credentials(credentials.clone())
        .create()
        .await
        .unwrap();

    let message = chat_completion.choices.first().unwrap().message.clone();

    Ok(message.content.unwrap().trim().to_string())
}
