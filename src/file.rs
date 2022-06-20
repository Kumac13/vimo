use std::io;

pub trait FileManagement {
    fn exists(&self) -> bool;
    fn create(&self) -> io::Result<()>;
    fn write(&self, text: String) -> io::Result<()>;
    fn read(&self) -> io::Result<String>;
}
