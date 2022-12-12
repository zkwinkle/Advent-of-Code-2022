use std::{cell::RefCell, iter::Peekable, rc::Rc};

use crate::tooling::SolutionResult;

type RcCell<T> = Rc<RefCell<T>>;
const MAX_SIZE: usize = 100000;

enum File {
    Data {
        name: String,
        data: usize,
    },
    Dir {
        name: String,
        files: Vec<RcCell<File>>,
        parent: RcCell<File>,
    },
    Root {
        files: Vec<RcCell<File>>,
    },
}

impl File {
    fn get_name<'a>(&'a self) -> &'a str {
        match self {
            File::Data { name, .. } => name,
            File::Dir { name, .. } => name,
            File::Root { .. } => "/",
        }
    }

    fn get_parent(&self) -> RcCell<File> {
        match self {
            File::Dir { parent, .. } => Rc::clone(parent),
            _ => panic!("Called get_parent on non-dir file"),
        }
    }

    /// Parsing function for files after 'ls' command. 'self' must be currently
    /// selected dir on which 'ls' is called. The iterator must not include any
    /// commands only files returned by 'ls'.
    fn ls(parent: RcCell<File>, file_lines: &mut Peekable<impl Iterator<Item = &str>>) {
        //let mut file_lines = file_lines.peekable();
        match *parent.borrow_mut() {
            File::Dir { ref mut files, .. } | File::Root { ref mut files } => {
                while let Some(line) = file_lines.next_if(|s| !s.starts_with('$')) {
                    let mut file_info = line.split_whitespace();
                    match file_info.next().unwrap() {
                        "dir" => files.push(Rc::new(RefCell::new(File::Dir {
                            name: file_info.next().unwrap().to_string(),
                            files: Vec::new(),
                            parent: Rc::clone(&parent),
                        }))),
                        size @ _ => files.push(Rc::new(RefCell::new(File::Data {
                            name: file_info.next().unwrap().to_string(),
                            data: size.parse().unwrap(),
                        }))),
                    }
                }
            }
            _ => {
                panic!("Called 'ls' on non-dir file");
            }
        }
    }

    fn cd<'a>(&self, into: &str, root: RcCell<File>) -> RcCell<File> {
        match self {
            File::Dir { .. } | File::Root { .. } => match into {
                "/" => root,
                ".." => self.get_parent(),
                name @ _ => self.get_child(name),
            },
            _ => {
                panic!("Called 'cd' on non-dir file");
            }
        }
    }

    fn get_child(&self, name: &str) -> RcCell<File> {
        match self {
            File::Dir { files, .. } | File::Root { files } => {
                let file = files
                    .iter()
                    .find(|&file| file.borrow().get_name() == name)
                    .unwrap();
                Rc::clone(file)
            }
            _ => {
                panic!("Called find_child on non-dir file");
            }
        }
    }

    fn get_size(&self) -> usize {
        match self {
            File::Dir { files, .. } | File::Root { files } => files
                .iter()
                .fold(0, |total, file| total + file.borrow().get_size()),
            File::Data { data, .. } => *data,
        }
    }
}

fn sum_under_max(dir: &File) -> usize {
    match dir {
        File::Root { files } | File::Dir { files, .. } => files.iter().fold(0, |acc, file| {
            let file = file.borrow();
            acc + match *file {
                File::Dir { .. } | File::Root { .. } => {
                    let size = file.get_size();
                    sum_under_max(&*file)
                        + if size < MAX_SIZE {
                            //println!("Dir '{}' under max size: {}", file.get_name(), size);
                            size
                        } else {
                            0
                        }
                }
                File::Data { .. } => 0,
            }
        }),
        _ => {
            panic!("WTF")
        }
    }
}

pub fn task1(input: &str) -> SolutionResult {
    let root: RcCell<File> = Rc::new(RefCell::new(File::Root { files: Vec::new() }));

    let mut current_dir: RcCell<File> = Rc::clone(&root);

    // parsing
    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        //println!("{}", line);
        let mut command = line.split_whitespace();
        match command.nth(1).unwrap() {
            "cd" => {
                current_dir = Rc::clone(&current_dir)
                    .borrow()
                    .cd(command.next().unwrap(), Rc::clone(&root));
            }
            "ls" => File::ls(Rc::clone(&current_dir), lines.by_ref()),
            _ => panic!("oops"),
        };
    }
    let res = SolutionResult::Unsigned(sum_under_max(&*root.borrow()));
    res
}

pub fn task2(input: &str) -> SolutionResult {
    SolutionResult::Unsigned(0)
}
