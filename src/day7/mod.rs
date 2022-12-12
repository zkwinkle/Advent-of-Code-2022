use std::{cmp::min, collections::HashMap, iter::Peekable};

use crate::tooling::SolutionResult;

const MAX_SIZE: usize = 100000;
const SPACE_AVAILABLE: usize = 70000000;
const SPACE_NEEDED: usize = 30000000;

#[derive(Debug)]
enum File<'a> {
    Data {
        data: usize,
    },
    Dir {
        files: HashMap<&'a str, File<'a>>,
        size: Option<usize>, // sizes have to be initialized
    },
    Root {
        files: HashMap<&'a str, File<'a>>,
        size: Option<usize>,
    },
}

impl<'a> File<'a> {
    fn new_dir() -> File<'a> {
        File::Dir {
            files: HashMap::with_capacity(32),
            size: None,
        }
    }

    fn new_data(data: usize) -> File<'a> {
        File::Data { data }
    }

    fn get_files(&self) -> &HashMap<&'a str, File<'a>> {
        match self {
            File::Dir { ref files, .. } | File::Root { ref files, .. } => files,
            File::Data { .. } => panic!("Called get_files on non-dir file"),
        }
    }

    fn get_files_mut(&mut self) -> &mut HashMap<&'a str, File<'a>> {
        match self {
            File::Dir { ref mut files, .. } | File::Root { ref mut files, .. } => files,
            File::Data { .. } => panic!("Called get_files_mut on non-dir file"),
        }
    }

    /// Parsing function for files after 'ls' command. 'self' must be currently
    /// selected dir on which 'ls' is called. The iterator must not include any
    /// commands only files returned by 'ls'.
    fn ls(&mut self, file_lines: &mut Peekable<impl Iterator<Item = &'a str>>) {
        let files = self.get_files_mut();

        while let Some(line) = file_lines.next_if(|s| !s.starts_with('$')) {
            let mut file_info = line.split_whitespace();
            let first = file_info.next().unwrap();
            let name = file_info.next().unwrap();
            match first {
                "dir" => files.insert(name, File::new_dir()),
                size => files.insert(name, File::new_data(size.parse().unwrap())),
            };
        }
    }

    //fn find_file(root: &'a File<'a>, path: Vec<&str>) -> &'a File<'a> {
    //    let mut current_file = root;
    //    for file in path {
    //        current_file = current_file
    //            .get_files()
    //            .iter()
    //            .find(|f| f.get_name() == file)
    //            .unwrap();
    //    }
    //    current_file
    //}

    fn find_file_mut<'b>(root: &'b mut File<'a>, path: &Vec<&'a str>) -> &'b mut File<'a> {
        let mut current_file = root;
        for file in path {
            current_file = current_file.get_files_mut().get_mut(file).unwrap();
        }
        current_file
    }

    /// self must be dir-like (Root or Dir)
    fn cd<'s>(into: &'s str, current_path: &mut Vec<&'s str>) {
        match into {
            "/" => current_path.clear(),
            ".." => {
                current_path.pop();
            }
            name => current_path.push(name),
        }
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
                        .iter_mut()
                        .fold(0, |total, (_, file)| total + file.init_sizes()),
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
    files.iter().fold(0, |acc, (_, file)| {
        acc + match *file {
            File::Dir { .. } | File::Root { .. } => {
                let size = file.get_size();
                sum_under_max(file)
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

fn parse(input: &str) -> File {
    let mut root: File = File::Root {
        files: HashMap::with_capacity(32),
        size: None,
    };

    let mut current_dir: Vec<&str> = Vec::with_capacity(32);

    // parsing
    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        //println!("{}", line);
        let mut command = line.split_whitespace();
        match command.nth(1).unwrap() {
            "cd" => {
                File::cd(command.next().unwrap(), &mut current_dir);
            }
            "ls" => File::find_file_mut(&mut root, &current_dir).ls(lines.by_ref()),
            _ => panic!("oops"),
        };
    }

    root.init_sizes();
    root
}

pub fn task1(input: &str) -> SolutionResult {
    let root = parse(input);

    SolutionResult::Unsigned(sum_under_max(&root))
}

fn get_smallest_over_thresh(dir: &File, thresh: usize) -> usize {
    let files = dir.get_files();

    files.iter().fold(SPACE_NEEDED, |acc, (_, file)| {
        match *file {
            File::Dir { .. } | File::Root { .. } => {
                let size = file.get_size();
                if size > thresh {
                    //println!("Dir '{}' over thresh: {}", file.get_name(), size);
                    let sub_size = get_smallest_over_thresh(file, thresh);
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

    let thresh = root.get_size() - (SPACE_AVAILABLE - SPACE_NEEDED);
    SolutionResult::Unsigned(get_smallest_over_thresh(&root, thresh))
}
