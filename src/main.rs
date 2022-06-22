mod cli;
mod config;
mod dir;
mod file;
mod memo;
use clap::Parser;
use cli::AppArg;
use config::Config;
use file::FileManagement;
use memo::*;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // parse arg
    let arg: AppArg = AppArg::parse();
    let file_name_option = arg.name;

    let config = Config::new();

    // create config
    if !config.exists() {
        config.set_config_file()?;
    }

    // read config
    let config_path = PathBuf::from(config.set_config_path());

    let memo: Memo = Memo::new(config_path, file_name_option);

    memo.open()?;

    Ok(())
}
