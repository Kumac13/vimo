use std::io;

pub trait DirectoryManagement {
    fn dir_exists(&self) -> bool;
    fn dir_create(&self) -> io::Result<()>;
}
