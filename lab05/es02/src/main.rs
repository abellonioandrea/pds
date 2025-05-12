use std::cell::RefCell;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::{Rc, Weak};

use std::fs;

use walkdir::WalkDir;

pub enum FSItem {
    Directory(Directory), // Directory contiene nome, i figli, eventuali metadati, il padre
    File(File), // File contiene il nome, eventuali metadati (es dimensione, owner, ecc), il padre
    SymLink(Link), // Il link simbolico contiene il Path a cui punta e il padre
}

impl FSItem {
    // These methods allow us to use an FSItem in a uniform way
    // regardless of its actual type.
    pub fn name(&self) -> &str {
        match self {
            FSItem::File(f) => &f.name,
            FSItem::Directory(d) => &d.name,
            FSItem::SymLink(s) => &s.name,
        }
    }

    pub fn parent(&self) -> FSNodeWeak {
        match self {
            FSItem::File(f) => f.parent.clone(),
            FSItem::Directory(d) => d.parent.clone(),
            FSItem::SymLink(l) => l.parent.clone(),
        }
    }

    pub fn get_children(&self) -> Option<&Vec<FSNode>> {
        match self {
            FSItem::Directory(d) => Some(&d.children),
            _ => None,
        }
    }

    // can be called only if you are sure that self is a directory
    pub fn add(&mut self, item: FSNode) {
        match self {
            FSItem::Directory(d) => {
                d.children.push(item);
            }
            _ => panic!("Cannot add item to non-directory"),
        }
    }

    pub fn remove(&mut self, name: &str) {
        match self {
            FSItem::Directory(d) => {
                d.children.retain(|child| child.borrow().name() != name);
            }
            _ => panic!("Cannot remove item from non-directory"),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        match self {
            FSItem::File(f) => f.name = name.to_owned(),
            FSItem::Directory(d) => d.name = name.to_owned(),
            FSItem::SymLink(s) => s.name = name.to_owned(),
        }
    }

    // return the absolute path of the item (of the parent)
    pub fn abs_path(&self) -> String {
        let mut parts = vec![];
        let mut current = self.parent().upgrade();

        while let Some(node) = current {
            let name = node.borrow().name().to_string();
            parts.insert(0, name);
            current = node.borrow().parent().upgrade();
        }

        if parts.len() < 2 {
            return "/".to_string();
        } else {
            return parts.join("/");
        }
    }
}

struct FileSystem {}

impl FileSystem {
    // crea un nuovo FS vuoto
    pub fn new() -> Self {
        unimplemented!()
    }

    // crea un nuovo FS replicando la struttura su disco
    pub fn from_disk() -> Self {
        unimplemented!()
    }

    // cambia la directory corrente, path come in tutti gli altri metodi
    // può essere assoluto o relativo;
    // es: “../sibling” vuol dire torna su di uno e scendi in sibling
    pub fn change_dir(&mut self, path: String) -> Result {
        unimplemented!()
    }

    // crea la dir in memoria e su disco
    pub fn make_dir(&self, path: String, name: String) -> Result {
        unimplemented!()
    }

    // crea un file vuoto in memoria e su disco
    pub fn make_file(&self, path: String, name: String) -> Result {
        unimplemented!()
    }

    // rinonima file / dir in memoria e su disco
    pub fn rename(&self, path: String, new_name: String) -> Result {
        unimplemented!()
    }

    // cancella file / dir in memoria e su disco, se è una dir cancella tutto il contenuto
    pub fn delete(&self, path: String) -> Result {
        unimplemented!()
    }

    // cerca l’elemento indicato dal path e restituisci un riferimento
    pub fn find(&self, path: String) -> Result {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_file_system_with_structure() -> FileSystem {
        let mut fs = FileSystem::new();
        fs.make_dir("/", "home").unwrap();
        fs.change_dir("/home").unwrap();
        fs.make_dir(".", "user").unwrap();
        fs.change_dir("./user").unwrap();
        fs.make_file(".", "file.txt").unwrap();
        fs.make_file(".", "file1.txt").unwrap();
        fs.make_dir("..", "user1").unwrap();
        fs.change_dir("../user1").unwrap();
        fs.make_file(".", "file.txt").unwrap();
        fs.make_link("/home", "link_user", "/home/user").unwrap();
        fs
    }

    #[test]
    fn create_basic_file_system() {
        let fs = FileSystem::new();
        assert_eq!(fs.root.borrow().name(), "");
    }

    #[test]
    fn create_directory() {
        let mut fs = FileSystem::new();
        fs.make_dir("/", "home").unwrap();
        let root = fs.root.borrow();
        if let Some(children) = root.get_children() {
            assert_eq!(children.len(), 1);
            assert_eq!(children[0].borrow().name(), "home");
        } else {
            panic!("Root should have children");
        }
    }


    #[test]
    fn test_file_system() {
        let fs = create_file_system_with_structure();
        assert!(fs.find("/home/user/file1.txt").is_some());
        assert!(fs.find("/home/demo/file.txt").is_none());
        assert!(fs.find("/home/user1/file.txt").is_some());
    }


    #[test]
    fn test_follow_link() {
        let mut fs = create_file_system_with_structure();
        let link = fs.find("/home/link_user/file.txt");
        assert!(link.is_some());

        fs.make_link("/home", "dead_link", "/home/dead").unwrap();
        let link = fs.find("/home/dead_link/filed.txt");
        assert!(link.is_none());
    }

    #[test]
    fn test_side_effects() {
        let mut fs = FileSystem::new();
        fs.set_side_effects(true);
        fs.set_real_path("/tmp"); //fs real path
        fs.make_dir("/", "test_dir").unwrap();
        fs.make_dir("/test_dir", "dir1").unwrap();
        fs.make_file("/test_dir/dir1", "file1.txt").unwrap();
        fs.make_file("/test_dir/dir1", "file2.txt").unwrap();
        fs.rename("/test_dir/dir1/file2.txt", "file3.txt").unwrap();
        fs.make_link("/test_dir/dir1", "link3.txt", "./file3.txt").unwrap();
        fs.make_link("/test_dir/dir1", "link1.txt", "./file1.txt").unwrap();
        fs.delete("/test_dir/dir1").unwrap();

        // uncommento to delete all
        // fs.delete("/test_dir").unwrap();

        assert!(true);
    }

    #[test]
    fn test_from_file_system() {

        // adjust to your system
        let fs = FileSystem::from_file_system("/etc/apt");
        assert!(fs.find("/sources.list").is_some());
    }
}


fn main() {
    println!("Hello, world!");
}
