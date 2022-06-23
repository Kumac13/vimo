use crate::file::FileManagement;
use dialoguer::theme::ColorfulTheme;
use dialoguer::*;
use home;
use std::fs::{metadata, read_to_string, File, OpenOptions};
use std::io;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

pub struct Config {
    path: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        Config {
            path: config_path_for().unwrap(),
        }
    }
    pub fn set_config_file(&self) -> io::Result<()> {
        let theme = ColorfulTheme::default();

        let input = Input::with_theme(&theme)
            .with_prompt("Absolute path to your jourlanl directory")
            .interact()?;

        let _ = &self.write(input);
        Ok(())
    }
    pub fn set_config_path(self) -> PathBuf {
        if !self.exists() {
            self.set_config_file().unwrap();
        }
        PathBuf::from(self.read().unwrap())
    }
}

impl FileManagement for Config {
    fn exists(&self) -> bool {
        metadata(&self.path).is_ok()
    }

    fn create(&self) -> io::Result<()> {
        match OpenOptions::new().create(true).write(true).open(&self.path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn read(&self) -> io::Result<String> {
        let mut contents = read_to_string(&self.path)?;

        if contents.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File is empty: {}", &self.path.display()),
            ));
        } else if contents.ends_with('\n') {
            contents.pop().ok_or_else(|| {
                io::Error::new(ErrorKind::Other, "Unable to remove last char from file")
            })?;
        }
        Ok(contents)
    }

    fn write(&self, text: String) -> io::Result<()> {
        let mut file = File::create(&self.path)?;
        file.write_all(text.as_bytes())
    }
}

pub fn config_path_for() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".vimo");
        path
    })
}
