use std::path::PathBuf;

use clap::Parser;

mod run;
mod setup;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
enum Command {
    Run { src: PathBuf },
    Setup,
}

impl Command {
    pub async fn run(self) -> Result<(), mlua::Error> {
        match self {
            Command::Run { src } => run::run(src).await,
            Command::Setup => setup::run().await,
        }
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), mlua::Error> {
    Command::parse().run().await
}
