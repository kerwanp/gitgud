pub mod error;

pub mod config;
pub mod prompt;
pub mod utils;

pub mod cli;

#[tokio::main]
pub async fn main() {
    dotenvy::dotenv().ok();

    if let Err(err) = cli::exec().await {
        eprintln!("\n\n🙀 {}", err);
    }
}
