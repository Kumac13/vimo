use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct AppArg {
    /// Optional File name
    #[clap(short, long, value_parser)]
    pub name: Option<String>,

    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    DATE,
}
