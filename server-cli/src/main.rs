use clap::{CommandFactory, Parser};

mod commands;

#[derive(clap::Parser)]
#[command(name = "cli")]
struct Cli {
    #[command(subcommand)]
    command: Option<commands::Command>,
}

impl Cli {
    fn execute() {
        match Self::parse().command {
            Some(command) => command.execute(),
            _ => Self::command().print_help().unwrap(),
        }
    }
}

fn main() {
    #[cfg(debug_assertions)]
    init_tracing();

    dotenvy::dotenv().ok();

    Cli::execute();
}

#[cfg(debug_assertions)]
fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();
}
