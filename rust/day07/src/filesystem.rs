use std::{io::BufRead, rc::Rc};

use crate::filesystem_entry::FilesystemEntry;

#[derive(Debug)]
pub struct Filesystem {
    root: Rc<FilesystemEntry>,
    current_working_directory: Rc<FilesystemEntry>,
}

impl Filesystem {
    pub fn new() -> Self {
        let root = Rc::new(FilesystemEntry::new_directory(None));

        Self {
            root: Rc::clone(&root),
            current_working_directory: root, //RefCell::new(root),
        }
    }

    pub fn build_with_descriptor(&mut self, descriptor: &mut dyn BufRead) {
        for line in descriptor.lines().map(|line| line.unwrap()) {
            if line.starts_with("$") {
                // Handle command
                if line.starts_with("$ cd") {
                    self.change_directory(line.strip_prefix("$ cd ").unwrap());
                }
            } else {
                // Add entry
                if line.starts_with("dir") {
                    let name = line.strip_prefix("dir ").map(String::from).unwrap();

                    self.make_directory(name);
                } else {
                    let mut line = line.split_whitespace();
                    let size: u32 = line.next().and_then(|size| size.parse().ok()).unwrap();
                    let name = line.next().map(String::from).unwrap();

                    self.touch(name, size);
                }
            }
        }
    }

    pub fn size(&self) -> u32 {
        self.root.size()
    }

    pub fn fold<A, F>(&self, init: A, f: &mut F) -> A
    where
        F: FnMut(&mut A, &Rc<FilesystemEntry>) -> A,
    {
        let mut accumulator = init;

        fn traverse<A, F>(accumulator: &mut A, f: &mut F, entry: &Rc<FilesystemEntry>)
        where
            F: FnMut(&mut A, &Rc<FilesystemEntry>) -> A,
        {
            *accumulator = f(accumulator, entry);

            if let Some(directories) = entry.directories() {
                directories.for_each(|dir| traverse(accumulator, f, &dir));
            }
        }

        traverse(&mut accumulator, f, &self.root);

        accumulator
    }

    fn change_directory(&mut self, directory: &str) {
        let new_directory = match directory {
            "/" => Some(Rc::clone(&self.root)),
            ".." => self.current_working_directory.parent(),
            child => self.current_working_directory.find_child(child),
        };

        new_directory.map(|dir| self.current_working_directory = dir);
    }

    fn make_directory(&self, name: String) {
        if let FilesystemEntry::Directory { entries, .. } = self.current_working_directory.as_ref()
        {
            let cwd = Rc::clone(&self.current_working_directory);
            let new_directory = FilesystemEntry::new_directory(Some(cwd));

            entries.borrow_mut().insert(name, Rc::new(new_directory));
        }
    }

    fn touch(&self, name: String, size: u32) {
        if let FilesystemEntry::Directory { entries, .. } = self.current_working_directory.as_ref()
        {
            let cwd = Rc::clone(&self.current_working_directory);
            let new_file = FilesystemEntry::new_file(size, Some(cwd));

            entries.borrow_mut().insert(name, Rc::new(new_file));
        }
    }
}
