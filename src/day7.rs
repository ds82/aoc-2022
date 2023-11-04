use crate::utils;
use std::iter::Iterator;
use std::{cell::RefCell, error::Error, rc::Rc, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

const ROOT_DIR: &str = "<ROOT>";
const TOTAL_SIZE: usize = 70000000;
const SPACE_NEEDED: usize = 30000000;

pub fn run() {
    day_7_1();
    day_7_2();
}

#[derive(Debug, Clone)]
struct RuntimeError(String);

#[derive(Debug)]
enum HistoryEntry {
    Command(Command),
    Result(Vec<Entry>),
}

impl HistoryEntry {
    fn add_to(&self, history: &mut History) -> () {
        match self {
            HistoryEntry::Command(c) => {
                history.add(HistoryEntry::Command(c.clone()));
            }
            HistoryEntry::Result(r) => {
                history.add(HistoryEntry::Result(r.clone()));
            }
        }
    }
}

#[derive(Debug)]
struct History {
    list: Vec<HistoryEntry>,
}

impl History {
    fn new() -> Self {
        History { list: vec![] }
    }

    fn add(&mut self, entry: HistoryEntry) -> () {
        self.list.push(entry);
    }
}

#[derive(Clone, Debug)]
enum Command {
    CD(String),
    LS,
    Unknown,
}

impl Command {
    fn parse(s: String) -> Self {
        let mut iter = s.split_whitespace().skip(1);
        let cmd = iter.next().unwrap();

        match cmd {
            "cd" => {
                let arg = iter.next().unwrap();
                Command::CD(arg.into())
            }
            "ls" => Command::LS,
            _ => Command::Unknown,
        }
    }
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
    parent: Option<usize>,
    idx: usize,
}

impl Dir {
    fn new(name: String, parent: Option<usize>, idx: Option<usize>) -> Self {
        Dir {
            name,
            parent,
            idx: idx.unwrap_or(0),
        }
    }

    fn has_parent(&self) -> bool {
        return self.parent.is_some();
    }

    fn size(&self, fs: &Filesystem) -> usize {
        fs.tree
            .iter()
            .filter(|e| e.is_parent(self.idx))
            .map(|e| e.size(fs))
            .sum()
    }
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    parent: Option<usize>,
    size: usize,
    idx: usize,
}

impl File {
    fn new(name: String, size: usize, parent: Option<usize>, idx: Option<usize>) -> Self {
        File {
            name,
            size,
            parent,
            idx: idx.unwrap_or(0),
        }
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

#[derive(Clone, Debug)]
enum Entry {
    Dir(Dir),
    File(File),
}

impl Entry {
    fn touch(name: String, size: usize, parent: Option<usize>) -> Self {
        Entry::File(File::new(name, size, parent, None))
    }

    fn mkdir(name: String, parent: Option<usize>, idx: Option<usize>) -> Self {
        Entry::Dir(Dir::new(name, parent, idx))
    }

    fn parse(line: String) -> Self {
        if line.starts_with("dir ") {
            Entry::Dir(Dir::new(line.trim_left_matches("dir ").into(), None, None))
        } else {
            let mut iter = line.split_whitespace();
            let size = iter.next().unwrap().parse::<usize>().unwrap();
            let name = iter.next().unwrap();

            Entry::File(File::new(name.into(), size, None, None))
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            Entry::Dir(_) => true,
            _ => false,
        }
    }

    fn name(&self) -> String {
        match self {
            Entry::Dir(d) => d.name.clone(),
            Entry::File(f) => f.name.clone(),
        }
    }
    fn parent(&self) -> Option<usize> {
        match self {
            Entry::Dir(d) => d.parent.clone(),
            Entry::File(f) => f.parent.clone(),
        }
    }

    fn is_parent(&self, parent: usize) -> bool {
        match self {
            Entry::Dir(d) => d.parent.is_some() && d.parent.unwrap() == parent,
            Entry::File(f) => f.parent.is_some() && f.parent.unwrap() == parent,
        }
    }

    fn dir(&self) -> &Dir {
        if let Entry::Dir(d) = self {
            return d;
        }
        panic!("Not a dir");
    }

    fn file(&self) -> &File {
        if let Entry::File(f) = self {
            return f;
        }
        panic!("Not a file");
    }

    fn size(&self, fs: &Filesystem) -> usize {
        match self {
            Entry::Dir(d) => d.size(fs),
            Entry::File(f) => f.size,
        }
    }
}

impl From<Dir> for Entry {
    fn from(value: Dir) -> Self {
        Entry::Dir(value)
    }
}
impl From<File> for Entry {
    fn from(value: File) -> Self {
        Entry::File(value)
    }
}

#[derive(Debug)]
struct Filesystem {
    tree: Vec<Entry>,
    idx: usize,
}

impl Filesystem {
    fn new() -> Self {
        let root = make_root(ROOT_DIR.into());
        Filesystem {
            idx: 0,
            tree: vec![root.into()],
        }
    }

    fn cd_to(&mut self, name: String) -> Result<()> {
        //TODO: refactor not to use enumerate but idx of entity!
        let target = self
            .tree
            .iter()
            .enumerate()
            .filter(|(_idx, e)| e.is_dir() && e.parent().unwrap_or(999999) == self.idx)
            .find(|(_idx, entry)| entry.name() == name);

        if let Some((idx, subdir)) = target {
            self.idx = idx;
            println!(
                "Now in dir: <{} [{}] | parent: {}>",
                self.idx,
                self.get_full_path(self.idx).unwrap(),
                self.get_by_idx(self.idx).parent.unwrap_or(99999)
            );
            return Ok(());
        }

        Err((anyhow::anyhow!("Cannot cd into subdir")))
    }

    fn cd_dotdot(&mut self) -> Result<()> {
        let target = self.tree.get(self.idx);

        if let Some(entry) = target {
            if let Entry::Dir(dir) = entry {
                self.idx = dir.parent.unwrap_or_else(|| panic!("Could not cd.."));
                return Ok(());
            }
        }

        return Err(anyhow::anyhow!("Cannot cd.."));
    }

    fn get_next_idx(&self) -> usize {
        self.tree.len()
    }

    fn add_results(&mut self, results: Vec<Entry>) -> () {
        for entry in results {
            self.add(entry)
        }
    }

    fn add(&mut self, entry: Entry) -> () {
        if let Entry::Dir(dir) = entry {
            let dir = Dir::new(dir.name, Some(self.idx), Some(self.get_next_idx()));
            println!(
                "Add dir {} to <{} [{}]>",
                dir.name,
                self.idx,
                self.get_full_path(self.idx).unwrap_or("?".into())
            );
            self.tree.push(dir.into());
        } else if let Entry::File(file) = entry {
            let file = File::new(
                file.name,
                file.size,
                Some(self.idx),
                Some(self.get_next_idx()),
            );
            println!(
                "Add file {} to <{} [{}]>",
                file.name,
                self.idx,
                self.get_full_path(self.idx).unwrap_or("?".into())
            );
            self.tree.push(file.into());
        } else {
            panic!("Cound not add unknown Entry");
        }
    }

    fn get_root_dir(&self) -> &Dir {
        self.tree[0].dir()
    }
    fn get_root_entry(&self) -> &Entry {
        &self.tree[0]
    }

    fn get_by_idx(&self, idx: usize) -> &Dir {
        self.tree[idx].dir()
    }

    fn get_full_path(&self, idx: usize) -> Result<String> {
        let mut path = vec![self.tree[idx].name()];
        let mut idx = idx;

        while let Some(parent) = self.tree[idx].parent() {
            path.push(self.tree[parent].name());
            idx = parent;
        }

        Ok(path.iter().rev().join("/"))
    }

    fn dirs(&self) -> Vec<&Dir> {
        self.tree
            .iter()
            .filter(|e| e.is_dir())
            .map(|d| d.dir())
            .collect()
    }

    fn size(&self, entry: &Entry) -> usize {
        match entry {
            Entry::Dir(d) => d.size(self),
            Entry::File(f) => f.size,
        }
    }
}

fn make_root(name: String) -> Dir {
    Dir::new(name, None, Some(0))
}

fn day_7_1() {
    let lines = utils::read_input_strings("_inputs/input-07-01.txt");

    let mut history = History::new();
    let mut result_buffer: Vec<Entry> = vec![];

    for line in lines.iter() {
        match line.chars().nth(0).unwrap() {
            '$' => {
                // if the result_buffer contains entries, these must be pushed to the history
                // they belong to the previous command
                if result_buffer.len() > 0 {
                    history.add(HistoryEntry::Result(result_buffer.clone()));
                    result_buffer.clear();
                }

                let cmd = Command::parse(line.clone());
                history.add(HistoryEntry::Command(cmd));
            }
            _ => {
                result_buffer.push(Entry::parse(line.clone()));
                continue;
            }
        }
    }
    // when we are done, we have to add the last result (if any) to the history
    if result_buffer.len() > 0 {
        history.add(HistoryEntry::Result(result_buffer.clone()));
        result_buffer.clear();
    }

    println!("=====================");
    println!("Now iterate over the history and fill our fs with that knowledge");
    println!("=====================");

    let mut fs = Filesystem::new();

    for entry in history.list.iter().skip(1 /* skip cd ROOT_DIR */) {
        match entry {
            HistoryEntry::Command(cmd) => {
                println!("cmd: {:?}", cmd);

                match cmd {
                    Command::CD(to) => {
                        if to == ".." {
                            fs.cd_dotdot();
                        } else {
                            fs.cd_to(to.clone());
                        }
                    }
                    Command::LS => {}
                    Command::Unknown => {}
                }
            }
            HistoryEntry::Result(r) => {
                fs.add_results(r.clone());
            }
        }
    }

    fs.dirs()
        .iter()
        .for_each(|e| println!("{}: {}", e.name, e.size(&fs)));

    // task 7.1
    let result = fs
        .dirs()
        .iter()
        .map(|e| e.size(&fs))
        .filter(|size| *size <= 100000)
        .sum::<usize>();

    println!(
        "Sum of directory size where each dir is at most 100000 in size: {}",
        result,
    );

    let total_used = fs.size(fs.get_root_entry());
    let total_free = TOTAL_SIZE - total_used;
    let needed = SPACE_NEEDED - total_free;
    println!("Total Space used: {}", total_used);
    println!("Free Space: {}", total_free);
    println!("Missing Space: {}", needed);

    // task 7.2
    let result_2 = fs
        .dirs()
        .iter()
        .map(|e| e.size(&fs))
        .sorted()
        .find(|size| *size >= needed);

    // println!("Delete Dir of {:?} to free enough space", result_2.unwrap());
    //

    println!("{:?}", result_2);
}

fn day_7_2() {}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_size_of_dir_with_two_files() {
    //     let mut root = make_root(ROOT_DIR.into());

    //     root.add_entry(Entry::touch("test.ext".into(), 1000));
    //     root.add_entry(Entry::touch("test2.ext".into(), 2000));

    //     assert_eq!(3000, root.size());
    // }

    #[test]
    // fn test_size_of_dir_with_dir_with_two_files() {
    //     let root = make_root(ROOT_DIR.into());

    //     root.add_entry(Entry::touch("test.ext".into(), 1000));
    //     root.add_entry(Entry::touch("test2.ext".into(), 2000));

    //     let mut subdir = Dir::new("subdir".into(), Some(root.clone()));
    //     subdir.add_entry(Entry::touch("some.txt".into(), 5000));
    //     subdir.add_entry(Entry::touch("some.txt".into(), 15000));

    //     root.add_entry(Entry::Dir(subdir));

    //     assert_eq!(23000, root.size());
    // }
    #[test]
    fn test_basic_fs() {
        let mut fs = Filesystem::new();
        let root_dir = fs.get_root();

        dbg!("root_dir: {}", root_dir);

        // shell.add_entry(Entry::mkdir("sub1", parent);
        // shell.add_results(vec![Entry::touch("test2.ext".into(), 2000)]);
    }
}
