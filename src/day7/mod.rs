use std::{
    cell::RefCell,
    cmp::min,
    iter::Peekable,
    rc::{Rc, Weak},
};

use crate::tooling::SolutionResult;

type RcCell<T> = Rc<RefCell<T>>;
type WeakCell<T> = Weak<RefCell<T>>;

const MAX_SIZE: usize = 100000;
const SPACE_AVAILABLE: usize = 70000000;
const SPACE_NEEDED: usize = 30000000;

#[derive(Debug)]
enum File<'a> {
    Data {
        name: &'a str,
        data: usize,
    },
    Dir {
        name: &'a str,
        files: Vec<RcCell<File<'a>>>,
        parent: WeakCell<File<'a>>,
        size: Option<usize>, // sizes have to be initialized
    },
    Root {
        files: Vec<RcCell<File<'a>>>,
        size: Option<usize>,
    },
}

impl<'a> File<'a> {
    fn new_dir(parent: RcCell<File<'a>>, name: &'a str) -> RcCell<File<'a>> {
        Rc::new(RefCell::new(File::Dir {
            name,
            files: Vec::with_capacity(32),
            parent: Rc::downgrade(&parent),
            size: None,
        }))
    }

    fn new_data(data: usize, name: &'a str) -> RcCell<File<'a>> {
        Rc::new(RefCell::new(File::Data { name, data }))
    }

    fn get_name(&self) -> &str {
        match self {
            File::Data { name, .. } => name,
            File::Dir { name, .. } => name,
            File::Root { .. } => "/",
        }
    }

    fn get_parent(&self) -> RcCell<File<'a>> {
        match self {
            File::Dir { parent, .. } => parent.upgrade().unwrap(),
            _ => panic!("Called get_parent on non-dir file"),
        }
    }

    fn get_files(&self) -> &Vec<RcCell<File<'a>>> {
        match self {
            File::Dir { ref files, .. } | File::Root { ref files, .. } => files,
            File::Data { .. } => panic!("Called get_files on non-dir file"),
        }
    }

    fn get_files_mut(&mut self) -> &mut Vec<RcCell<File<'a>>> {
        match self {
            File::Dir { ref mut files, .. } | File::Root { ref mut files, .. } => files,
            File::Data { .. } => panic!("Called get_files_mut on non-dir file"),
        }
    }

    /// Parsing function for files after 'ls' command. 'self' must be currently
    /// selected dir on which 'ls' is called. The iterator must not include any
    /// commands only files returned by 'ls'.
    fn ls(parent: RcCell<File<'a>>, file_lines: &mut Peekable<impl Iterator<Item = &'a str>>) {
        let mut binding = parent.borrow_mut();
        let files = binding.get_files_mut();

        while let Some(line) = file_lines.next_if(|s| !s.starts_with('$')) {
            let mut file_info = line.split_whitespace();
            let first = file_info.next().unwrap();
            let name = file_info.next().unwrap();
            match first {
                "dir" => files.push(File::new_dir(Rc::clone(&parent), name)),
                size => files.push(File::new_data(size.parse().unwrap(), name)),
            }
        }
    }

    /// self must be dir-like (Root or Dir)
    fn cd(&self, into: &str, root: RcCell<File<'a>>) -> RcCell<File<'a>> {
        match into {
            "/" => root,
            ".." => self.get_parent(),
            name => self.get_child(name),
        }
    }

    /// self must be dir-like (Root or Dir)
    fn get_child(&self, name: &str) -> RcCell<File<'a>> {
        let files = self.get_files();
        let file = files
            .iter()
            .find(|&file| file.borrow().get_name() == name)
            .unwrap();
        Rc::clone(file)
    }

    fn init_sizes(&mut self) -> usize {
        match self {
            File::Dir {
                files,
                ref mut size,
                ..
            }
            | File::Root {
                files,
                ref mut size,
                ..
            } => {
                *size = Some(
                    files
                        .iter()
                        .fold(0, |total, file| total + file.borrow_mut().init_sizes()),
                );
                size.unwrap()
            }
            File::Data { data, .. } => *data,
        }
    }

    fn get_size(&self) -> usize {
        match self {
            File::Dir { size, .. } | File::Root { size, .. } => {
                size.expect("Called get_size() on unitialized file directory")
            }
            File::Data { data, .. } => *data,
        }
    }
}

fn sum_under_max(dir: &File) -> usize {
    let files = dir.get_files();
    files.iter().fold(0, |acc, file| {
        let file = file.borrow();
        acc + match *file {
            File::Dir { .. } | File::Root { .. } => {
                let size = file.get_size();
                sum_under_max(&file)
                    + if size < MAX_SIZE {
                        //println!("Dir '{}' under max size: {}", file.get_name(), size);
                        size
                    } else {
                        0
                    }
            }
            File::Data { .. } => 0,
        }
    })
}

fn parse(input: &str) -> RcCell<File> {
    let root: RcCell<File> = Rc::new(RefCell::new(File::Root {
        files: Vec::with_capacity(32),
        size: None,
    }));

    let mut current_dir: RcCell<File> = Rc::clone(&root);

    // parsing
    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        //println!("{}", line);
        let mut command = line.split_whitespace();
        match command.nth(1).unwrap() {
            "cd" => {
                let new_dir = current_dir
                    .borrow()
                    .cd(command.next().unwrap(), Rc::clone(&root));
                current_dir = new_dir;
            }
            "ls" => File::ls(Rc::clone(&current_dir), lines.by_ref()),
            _ => panic!("oops"),
        };
    }

    root.borrow_mut().init_sizes();
    root
}

pub fn task1(input: &str) -> SolutionResult {
    let root = parse(input);

    let res = SolutionResult::Unsigned(sum_under_max(&root.borrow()));
    res
}

fn get_smallest_over_thresh(dir: &File, thresh: usize) -> usize {
    let files = dir.get_files();

    files.iter().fold(SPACE_NEEDED, |acc, file| {
        let file = file.borrow();
        match *file {
            File::Dir { .. } | File::Root { .. } => {
                let size = file.get_size();
                if size > thresh {
                    //println!("Dir '{}' over thresh: {}", file.get_name(), size);
                    let sub_size = get_smallest_over_thresh(&file, thresh);
                    min(size, sub_size)
                } else {
                    acc
                }
            }
            File::Data { .. } => acc,
        }
    })
}

pub fn task2(input: &str) -> SolutionResult {
    let root = parse(input);

    let thresh = root.borrow().get_size() - (SPACE_AVAILABLE - SPACE_NEEDED);
    let res = SolutionResult::Unsigned(get_smallest_over_thresh(&root.borrow(), thresh));
    res
}
