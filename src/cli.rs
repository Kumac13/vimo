use clap::Parser;

#[derive(Parser, Debug)]
pub struct AppArg {
    /// Optional File name
    #[clap(short, long, value_parser)]
    pub name: Option<String>,
    /// Add a monologue to today's memo
    #[clap(short, long, value_parser)]
    pub monologue: Option<String>,
}
