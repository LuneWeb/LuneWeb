use crate::start_application;
use clap::Parser;
use std::path::PathBuf;
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(clap::Subcommand)]
enum SubCommand {
    Run {
        input: PathBuf,
        javascript_inputs: Vec<PathBuf>,
    },
    Build,
}

pub async fn init() {
    let cli = Cli::parse();

    match cli.command {
        SubCommand::Run {
            input,
            javascript_inputs,
        } => {
            start_application(input, javascript_inputs).await;
        }
        SubCommand::Build => {
            unimplemented!()
        }
    }
}
