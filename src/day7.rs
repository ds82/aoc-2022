use crate::utils;
use std::{cell::RefCell, rc::Rc};
pub fn run() {
    day_7_1();
    day_7_2();
}

enum Command {
    CD(String),
    LS,
}

struct Path(Vec<String>);

impl Path {
    fn new(dir: Vec<String>) -> Self {
        Path(dir)
    }
}

struct Filename(String);

#[derive(Clone, Debug)]
struct Dir {
    name: String,
    entries: Vec<Entry>,
}

impl Dir {
    fn new(name: String) -> Self {
        Dir {
            name,
            entries: vec![],
        }
    }

    fn add_entries(&mut self, entries: &mut Vec<Entry>) {
        self.entries.append(entries);
    }

    fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> Self {
        File { name, size }
    }
}

trait Size {
    fn size(&self) -> usize;
}

impl Size for File {
    fn size(&self) -> usize {
        self.size
    }
}

impl Size for Dir {
    fn size(&self) -> usize {
        self.entries.iter().map(|e| e.size()).sum()
    }
}

#[derive(Clone, Debug)]
enum Entry {
    Dir(Dir),
    File(File),
}

impl Entry {
    fn touch(name: String, size: usize) -> Entry {
        Entry::File(File::new(name, size))
    }

    fn mkdir(name: String) -> Entry {
        Entry::Dir(Dir::new(name))
    }
}

impl Size for Entry {
    fn size(&self) -> usize {
        match self {
            Entry::Dir(dir) => dir.size(),
            Entry::File(file) => file.size(),
        }
    }
}

struct Shell {
    cwd: Rc<RefCell<Dir>>,
    root: Rc<RefCell<Dir>>,
}

impl Shell {
    fn new() -> Self {
        let root = make_root(ROOT_DIR.into());
        Shell {
            cwd: root.clone(),
            root,
        }
    }
}

const ROOT_DIR: &str = "/";

fn make_root(name: String) -> Rc<RefCell<Dir>> {
    Rc::new(RefCell::new(Dir::new(name)))
}

fn day_7_1() {
    let lines = utils::read_input_strings("_inputs/input-06-01.txt");
    let shell = Shell::new();
    // lines.iter().
}

fn day_7_2() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size_of_dir_with_two_files() {
        let root = make_root(ROOT_DIR.into());
        let mut dir = root.borrow_mut();

        dir.add_entry(Entry::touch("test.ext".into(), 1000));
        dir.add_entry(Entry::touch("test2.ext".into(), 2000));

        assert_eq!(3000, dir.size());
    }

    #[test]
    fn test_size_of_dir_with_dir_with_two_files() {
        let root = make_root(ROOT_DIR.into());
        let mut root_dir = root.borrow_mut();

        root_dir.add_entry(Entry::touch("test.ext".into(), 1000));
        root_dir.add_entry(Entry::touch("test2.ext".into(), 2000));

        let mut subdir = Dir::new("subdir".into());
        subdir.add_entry(Entry::touch("some.txt".into(), 5000));
        subdir.add_entry(Entry::touch("some.txt".into(), 15000));

        root_dir.add_entry(Entry::Dir(subdir));

        assert_eq!(23000, root_dir.size());
    }
}
