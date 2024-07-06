use app::App;
use clap::Parser;
use ctx::{Context, ContextBuilder};
use std::path::PathBuf;
use util::{lune_ctx, Error as LuneWebError};

mod app;
mod ctx;
mod util;

const INCORRECT_EXTENSION_ERROR: &str = "Provided input file must have a .luau extension";

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(clap::Subcommand)]
enum SubCommand {
    Run { input: PathBuf },
    Build,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        SubCommand::Run { input } => {
            let Some(ext) = input.extension() else {
                panic!("{INCORRECT_EXTENSION_ERROR}");
            };

            if ext != "luau" {
                panic!("{INCORRECT_EXTENSION_ERROR}");
            }

            App::from(
                ContextBuilder::new()
                    .with_lune_ctx(lune_ctx().expect("Failed to create GlobalsContextBuilder"))
                    .with_luau_input(&input),
            )
            .run()
            .await
            .expect("Failed to run application");
        }
        SubCommand::Build => {
            unimplemented!()
        }
    }
}
