// src/filesystem.rs

use no_space_left_on_device::directory::{Directory, DirectoryRef, File, Cmd, FileSystemObj};


/// Represents the entire filesystem, which is a linked tree of all the filesystem
/// objects. Contains the root node.
#[derive(Debug, Clone)]
pub struct FileSystem<'a>(pub DirRef<'a>);

impl<'a> TryFrom<Vec<Cmd<'a>>> for FileSystem<'a> {
    type Error = Error;

    fn try_from(commands: Vec<Cmd<'a>>) -> Result<Self, Self::Error> {
        let root = Rc::new(RefCell::new(Directory::from("/")));
        let mut open_dirs = vec![root.clone()];

        for command in commands {
            // This is safe, there's nothing in the loop that would cause us to
            // empty `open_dirs` that isn't already handled.
            let current_dir = open_dirs.last_mut().unwrap();

            // Based on the command we're looking at...
            match command {
                // Move into a new directory by getting that directory's reference
                // from the current directory's contents and pushing that reference
                // to the end of the list of open directories.
                Cmd::MoveIn(dir) => {
                    let dir = current_dir.borrow_mut().get_child(dir.label).unwrap();
                    open_dirs.push(dir);
                }

                // Move up out of the current directory by dropping the last directory
                // from the list of open directories.
                Cmd::MoveUp => {
                    open_dirs
                        .pop()
                        .ok_or(anyhow!("Cannot 'cd ..' out of root!"))?;
                }

                // Move to the root directory by dropping all but the first (root)
                // directory from the list of open directories.
                Cmd::MoveRoot => open_dirs.truncate(1),

                // Process a command to list contents by adding all the files and
                // directories listed as children of the currently open directory.
                Cmd::List(mut objs) => {
                    for obj in objs.drain(..) {
                        match obj {
                            FileSystemObj::Directory(d) => current_dir.borrow_mut().dirs.push(d),
                            FileSystemObj::File(f) => current_dir.borrow_mut().files.push(f),
                        }
                    }
                }
            }
        }
        Ok(FileSystem(root))
    }
}

impl FileSystem<'_> {
    /// Fill in the sizes of all the directories in the file system by recursively
    /// walking the file system.
    fn calculate_directory_sizes(&self) {
        // Recursively walk the file system tree and fill in the sizes for
        // each directory.
        fn size(dir: DirectoryRef) -> u32 {
            let mut total = 0;

            // Add the sizes of all the files in this directory
            for file in dir.borrow().files.iter() {
                total += file.size;
            }

            // Add (and fill in) recursively all the sub-directories in this
            // directory. This size() invocation is the recursive portion.
            for child_dir in dir.borrow().dirs.iter() {
                total += size(child_dir.clone());
            }

            // Update this directory
            dir.borrow_mut().size = total;

            total
        }

        // Perform the walk
        size(self.0.clone());
    }
}
