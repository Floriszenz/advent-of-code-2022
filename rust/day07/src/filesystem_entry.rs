use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub enum FilesystemEntry {
    Directory {
        entries: RefCell<HashMap<String, Rc<Self>>>,
        parent: Option<Rc<Self>>,
    },
    File {
        size: u32,
        parent: Option<Rc<Self>>,
    },
}

impl FilesystemEntry {
    pub fn new_directory(parent: Option<Rc<Self>>) -> Self {
        Self::Directory {
            entries: RefCell::new(HashMap::new()),
            parent,
        }
    }

    pub fn new_file(size: u32, parent: Option<Rc<Self>>) -> Self {
        Self::File { size, parent }
    }

    pub fn is_directory(&self) -> bool {
        match self {
            Self::File { .. } => false,
            Self::Directory { .. } => true,
        }
    }

    pub fn parent(&self) -> Option<Rc<Self>> {
        match self {
            Self::File { parent, .. } => parent.as_ref().map(Rc::clone),
            Self::Directory { parent, .. } => parent.as_ref().map(Rc::clone),
        }
    }

    pub fn children(&self) -> Option<Vec<Rc<Self>>> {
        if let Self::Directory { entries, .. } = self {
            Some(entries.borrow().values().map(Rc::clone).collect::<Vec<_>>())
        } else {
            None
        }
    }

    pub fn find_child(&self, name: &str) -> Option<Rc<Self>> {
        if let Self::Directory { entries, .. } = self {
            entries.borrow().get(name).map(Rc::clone)
        } else {
            None
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            Self::File { size, .. } => *size,
            Self::Directory { entries, .. } => {
                entries.borrow().values().map(|entry| entry.size()).sum()
            }
        }
    }
}
