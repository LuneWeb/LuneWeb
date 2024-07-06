use app::App;
use clap::Parser;
use ctx::{Context, ContextBuilder};
use util::{lune_ctx, Error as LuneWebError};

mod app;
mod ctx;
mod util;

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(clap::Subcommand)]
enum SubCommand {
    Run,
    Build,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        SubCommand::Run => {
            App::from(
                ContextBuilder::new()
                    .with_lune_ctx(lune_ctx().expect("Failed to create GlobalsContextBuilder")),
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
