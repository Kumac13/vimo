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
        let today = Local::now().date().format("%Y-%m-%d").to_string();

        Memo {
            title: title.unwrap_or(today),
            root_path,
        }
    }

    pub fn file_path(&self) -> PathBuf {
        self.file_directory().join(format!("{}.md", &self.title))
    }

    pub fn file_directory(&self) -> PathBuf {
        let this_month = Local::now().date().format("%Y-%m").to_string();
        self.root_path.join(this_month)
    }

    pub fn ensure_file_exits(&self) -> io::Result<()> {
        if !self.dir_exists() {
            self.dir_create()?;
        }

        if !self.exists() {
            self.create()?;
            self.write(String::from(&self.title))?;
        }

        Ok(())
    }

    pub fn open(self) -> io::Result<()> {
        self.ensure_file_exits()?;

        env::set_current_dir(&self.root_path)?;
        duct::cmd("vim", vec![self.file_path().to_str().unwrap()]).run()?;
        Ok(())
    }

    pub fn write_monologue(&self, content: String) -> anyhow::Result<()> {
        self.ensure_file_exits()?;

        let path = self.file_path();
        let mut file_content = std::fs::read_to_string(&path)?;

        let current_time = Local::now().format("%H:%M").to_string();
        let new_content = format!("\n- {} {}", current_time, content);

        let monologue_section = "## Monologue";
        if let Some(index) = file_content.find(monologue_section) {
            // Find the position after '## Monologue' and its following newline
            let insert_position = index
                + monologue_section.len()
                + if file_content.chars().nth(index + monologue_section.len()) == Some('\n') {
                    1
                } else {
                    0
                };
            file_content.insert_str(insert_position, &new_content);
        } else {
            // If '## Monologue' section is not found, append it to the file with the new content
            file_content.push_str(&format!("\n{}\n{}", monologue_section, new_content));
        }

        std::fs::write(&path, file_content)?;
        Ok(())
    }
}

impl FileManagement for Memo {
    fn exists(&self) -> bool {
        metadata(&self.file_path()).is_ok()
    }
    fn create(&self) -> io::Result<()> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(self.file_path())?;
        Ok(())
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
        let mut file = File::create(self.file_path())?;
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
