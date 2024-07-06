use crate::start_application;
use clap::Parser;
use std::path::PathBuf;

const INCORRECT_EXTENSION_ERROR: &str = "Provided input file must have a .luau extension";

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(clap::Subcommand)]
enum SubCommand {
    Run { input: PathBuf },
    Build,
}

pub async fn init() {
    let cli = Cli::parse();

    match cli.command {
        SubCommand::Run { input } => {
            let Some(ext) = input.extension() else {
                panic!("{INCORRECT_EXTENSION_ERROR}");
            };

            if ext != "luau" {
                panic!("{INCORRECT_EXTENSION_ERROR}");
            }

            start_application(input).await;
        }
        SubCommand::Build => {
            unimplemented!()
        }
    }
}
