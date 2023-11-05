use std::rc::Rc;

use crate::{filesystem::Filesystem, filesystem_entry::FilesystemEntry};

pub struct FilesystemWalker {
    stack: Vec<Rc<FilesystemEntry>>,
}

impl FilesystemWalker {
    pub fn traverse(filesystem: &Filesystem) -> Self {
        Self {
            stack: vec![Rc::clone(filesystem.root())],
        }
    }
}

impl Iterator for FilesystemWalker {
    type Item = Rc<FilesystemEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().and_then(|entry| {
            if let Some(children) = entry.children() {
                for child in children.into_iter() {
                    self.stack.push(child);
                }
            }

            Some(entry)
        })
    }
}
