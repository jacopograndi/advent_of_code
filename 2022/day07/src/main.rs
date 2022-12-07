use std::{fs, str::FromStr};

#[derive(Debug, Clone)]
struct Dir {
    name: String,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u64,
}

#[derive(Debug, Clone)]
enum Node {
    Dir(Dir),
    File(File),
}

#[derive(Debug)]
enum Line {
    Cd(String),
    Ls,
    Node(Node),
}

impl FromStr for Line {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next() == Some('$') {
            let cmd = s.trim_start_matches('$').trim();
            if cmd.starts_with("cd") {
                let arg = cmd.trim_start_matches("cd").trim().to_string();
                Ok(Line::Cd(arg))
            } else {
                Ok(Line::Ls)
            }
        } else {
            if s.starts_with("dir") {
                let name = s.trim_start_matches("dir").trim().to_string();
                Ok(Line::Node(Node::Dir(Dir { name })))
            } else {
                let (dim, name) = s.split_once(" ").unwrap();
                Ok(Line::Node(Node::File(File {
                    name: name.to_string(),
                    size: dim.parse().unwrap(),
                })))
            }
        }
    }
}

#[derive(Debug)]
struct Log {
    lines: Vec<Line>,
}

impl FromStr for Log {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Log {
            lines: s
                .split("\n")
                .filter(|x| !x.is_empty())
                .map(|l| l.trim().parse::<Line>().unwrap())
                .collect(),
        })
    }
}

#[derive(Debug)]
enum FileTree {
    File(File),
    Branch((Dir, Vec<Box<FileTree>>)),
}

impl FileTree {
    fn from_log(log: &Log) -> FileTree {
        let mut path = vec![];
        let mut tree = FileTree::Branch((
            Dir {
                name: "/".to_string(),
            },
            vec![],
        ));
        for line in log.lines.iter() {
            match line {
                Line::Cd(dirname) => match dirname.as_str() {
                    "/" => path = vec!["/".to_string()],
                    ".." => {
                        path.pop();
                    }
                    oth => path.push(oth.to_string()),
                },
                Line::Ls => (),
                Line::Node(node) => {
                    let success = tree.insert(&node, path.clone());
                    if !success {
                        println!("{node:?}, {path:?}");
                    }
                }
            };
        }
        tree
    }

    fn insert(&mut self, node: &Node, path: Vec<String>) -> bool {
        match self {
            Self::Branch((root, v)) => {
                if path.len() == 1 && Some(&root.name) == path.first() {
                    match node {
                        Node::Dir(dir) => v.push(Box::new(FileTree::Branch((dir.clone(), vec![])))),
                        Node::File(file) => v.push(Box::new(FileTree::File(file.clone()))),
                    };
                    true
                } else if !path.is_empty() {
                    for child in v.iter_mut() {
                        let mut newpath = path.clone();
                        newpath.remove(0);
                        match &**child {
                            Self::Branch((oth, _)) => {
                                if Some(&oth.name) == newpath.first() {
                                    return child.insert(node, newpath);
                                } else {
                                    false
                                }
                            }
                            _ => false,
                        };
                    }
                    false
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn dir_size(&self) -> u64 {
        match self {
            Self::Branch((_, v)) => v.iter().map(|x| x.dir_size()).sum(),
            Self::File(file) => file.size,
        }
    }

    fn sum_sizes(&self, min: u64) -> u64 {
        match self {
            Self::Branch((_, v)) => {
                let size = self.dir_size();
                (if size <= min { size } else { 0 })
                    + v.iter().map(|x| x.sum_sizes(min)).sum::<u64>()
            }
            _ => 0,
        }
    }

    fn dirs_to_vec(&self, f: fn(&Self) -> u64) -> Vec<u64> {
        match self {
            Self::Branch((n, v)) => {
                let mut childs = v.iter().map(|x| x.dirs_to_vec(f)).flatten().collect();
                let mut ret = vec![f(self)];
                ret.append(&mut childs);
                ret
            }
            Self::File(_) => vec![],
        }
    }

    fn get_smallest(&self, tot: u64, min: u64) -> u64 {
        let avail = min - (tot - self.dir_size());
        self.dirs_to_vec(|t| t.dir_size())
            .iter()
            .filter(|d| **d >= avail)
            .min()
            .unwrap()
            .clone()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let log: Log = input.parse().unwrap();
    let tree = FileTree::from_log(&log);
    let size = tree.sum_sizes(100000);
    println!("Sum of directories with size <= 100000: {}", size);
    let size = tree.get_smallest(70000000, 30000000);
    println!("Sum of the smallest valid dir: {}", size);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn parse_log() {
        let input = fs::read_to_string("text.txt").unwrap();
        let log: Log = input.parse().unwrap();
        assert_eq!(log.lines.len(), 23);
    }

    #[test]
    fn big_directories() {
        let input = fs::read_to_string("text.txt").unwrap();
        let log: Log = input.parse().unwrap();
        let tree = FileTree::from_log(&log);
        let size = tree.sum_sizes(100000);
        assert_eq!(size, 95437);
    }

    #[test]
    fn smallest_to_delete() {
        let input = fs::read_to_string("text.txt").unwrap();
        let log: Log = input.parse().unwrap();
        let tree = FileTree::from_log(&log);
        let size = tree.get_smallest(70000000, 30000000);
        assert_eq!(size, 24933642);
    }
}
