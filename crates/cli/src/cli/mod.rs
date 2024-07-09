use clap::Parser;

use std::{
    env::{current_dir, set_current_dir},
    path::PathBuf,
};

mod home;
mod run;
mod setup;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(clap::Subcommand)]
enum SubCommand {
    /// Setup luneweb and install luau type definition files in the home directory
    Setup { dir: Option<PathBuf> },
    /// Runs luneweb
    Run { dir: Option<PathBuf> },
    /// Builds binary (unimplemented)
    Build,
}

fn set_cwd(dir: Option<PathBuf>) -> PathBuf {
    let cwd = current_dir().unwrap();
    let cwd = match dir {
        Some(dir) => cwd.join(dir),
        None => cwd,
    };

    set_current_dir(&cwd).unwrap();
    cwd
}

pub async fn init() {
    let cli = Cli::parse();

    match cli.command {
        SubCommand::Setup { dir } => setup::setup(dir),
        SubCommand::Run { dir } => run::run(dir).await,
        SubCommand::Build => {
            unimplemented!()
        }
    }
}
