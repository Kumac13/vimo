use clap::Parser;

#[derive(Parser, Debug)]
pub struct AppArg {
    /// Optional File name
    #[clap(short, long, value_parser)]
    pub name: Option<String>,
}
