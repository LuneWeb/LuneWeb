mod cli;
mod config;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const LUAU_TYPES: &str = include_str!("../../../globals.d.luau");

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    cli::init().await
}
