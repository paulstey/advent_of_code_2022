// src/directory.rs

use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Directory<'a> {
    label: &'a str,
    dirs: Vec<DirectoryRef<'a>>,
    files: Vec<File<'a>>,
    size: u32,
}

/// A type alias for a Directory wrapped in an Rc, which allows for the Directory to have
/// multiple owners, and RefCell, which allows for runtime-checked mutable borrows of the
/// Directory. Together, these two properties allow us to recursively nest `Directory`s
/// within other `Directory`s.
pub type DirectoryRef<'a> = Rc<RefCell<Directory<'a>>>;

impl<'a> Directory<'a> {
    pub fn from(label: &'a str) -> Self {
        let dirs = Vec::new();
        let files = Vec::new();
        let size = 0;

        Self {
            label,
            dirs,
            files,
            size,
        }
    }

    /// Search the contents of a file system object and return the child object
    /// indicated by `label`.
    fn get_child(&self, label: &str) -> Option<DirectoryRef<'a>> {
        self.dirs
            .iter()
            .find(|c| c.borrow().label == label)
            .cloned()
    }

    /// Add a nested directory to this directory
    fn add_directory(&mut self, dir: Directory<'a>) {
        let dir_ref = Rc::new(RefCell::new(dir));
        self.dirs.push(dir_ref);
    }
}

#[derive(Debug, Clone)]
pub struct File<'a> {
    name: &'a str,
    size: u32,
}

/// Enum to allow both types of items to be stored in a single vector. Both types
/// are in Rc<RefCell<T>> wrappers to TODO
#[derive(Debug, Clone)]
pub enum FileSystemObj<'a> {
    Directory(DirectoryRef<'a>),
    File(File<'a>),
}

/// Represents a command from the input file. Commands come in one of four flavors.
#[derive(Debug, Clone)]
pub enum Cmd<'a> {
    MoveIn(Directory<'a>),
    MoveUp,
    MoveRoot,
    List(Vec<FileSystemObj<'a>>),
}

