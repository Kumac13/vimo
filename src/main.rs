mod cli;
mod config;
mod dir;
mod file;
mod memo;

use clap::Parser;
use cli::AppArg;
use config::Config;
use memo::Memo;

fn main() -> anyhow::Result<()> {
    // parse arg
    let arg: AppArg = AppArg::parse();
    let file_name_option = arg.name;

    let config_path = Config::new().set_config_path();

    let memo: Memo = Memo::new(config_path, file_name_option);

    memo.open()?;

    Ok(())
}
