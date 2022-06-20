use crate::file::FileManagement;
use chrono::Local;
use std::fs::{metadata, read_to_string, File, OpenOptions};
use std::io;
use std::io::{ErrorKind, Write};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub struct Memo {
    pub title: String,
    pub root_path: PathBuf,
}

impl Memo {
    pub fn new(root_path: PathBuf, title: Option<String>) -> Memo {
        let today = Local::today().format("%Y-%m-%d").to_string();

        Memo {
            title: title.unwrap_or(today),
            root_path,
        }
    }

    pub fn file_path(&self) -> String {
        format!("{}/{}.md", &self.root_path.display(), &self.title)
    }

    pub fn open(self) -> io::Result<()> {
        if self.exists() {
            Command::new("vim").arg(&self.file_path()).exec();
            Ok(())
        } else {
            self.create()?;
            self.write(String::from("DAIRY TASK"))?;
            Command::new("vim").arg(&self.file_path()).exec();
            Ok(())
        }
    }
}

impl FileManagement for Memo {
    fn exists(&self) -> bool {
        let path = format!("{}/{}.md", &self.root_path.display(), &self.title);
        metadata(path).is_ok()
    }
    fn create(&self) -> io::Result<()> {
        let path = format!("{}/{}.md", &self.root_path.display(), &self.title);
        match OpenOptions::new().create(true).write(true).open(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    fn read(&self) -> io::Result<String> {
        let mut contents = read_to_string(&self.root_path)?;

        if contents.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File is empty: {}", &self.root_path.display()),
            ));
        } else if contents.ends_with('\n') {
            contents.pop().ok_or_else(|| {
                io::Error::new(ErrorKind::Other, "Unable to remove last char from file")
            })?;
        }
        Ok(contents)
    }
    fn write(&self, date_format: String) -> io::Result<()> {
        let title = format!(
            "# {}: {}\n",
            date_format,
            Local::today().format("%Y-%m-%d").to_string()
        );
        let mut file = File::create(&self.file_path())?;
        file.write_all(title.as_bytes())
    }
}
