use crate::dir::DirectoryManagement;
use crate::file::FileManagement;
use chrono::Local;
use std::env;
use std::fs::{create_dir_all, metadata, read_to_string, File, OpenOptions};
use std::io;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

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
        format!("{}/{}.md", &self.file_directory(), &self.title)
    }

    pub fn file_directory(&self) -> String {
        let this_month = Local::today().format("%Y-%m").to_string();
        format!("{}/{}", &self.root_path.display(), this_month)
    }

    pub fn open(self) -> io::Result<()> {
        if !self.dir_exists() {
            self.dir_create()?;
        }
        if self.exists() {
            env::set_current_dir(&self.root_path)?;
            duct::cmd("vim", vec![&self.file_path()]).run()?;
            Ok(())
        } else {
            self.create()?;
            self.write(String::from(&self.title))?;
            env::set_current_dir(&self.root_path)?;
            duct::cmd("vim", vec![&self.file_path()]).run()?;
            Ok(())
        }
    }
}

impl FileManagement for Memo {
    fn exists(&self) -> bool {
        metadata(&self.file_path()).is_ok()
    }
    fn create(&self) -> io::Result<()> {
        let path = format!("{}", &self.file_path());
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
    fn write(&self, text: String) -> io::Result<()> {
        let title = format!("# {}", &self.title);
        let mut file = File::create(&self.file_path())?;
        file.write_all(title.as_bytes())
    }
}

impl DirectoryManagement for Memo {
    fn dir_exists(&self) -> bool {
        metadata(&self.file_directory()).is_ok()
    }

    fn dir_create(&self) -> io::Result<()> {
        create_dir_all(&self.file_directory())
    }
}
