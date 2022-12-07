use aoc_runner_derive::aoc;

enum Entry {
    Directory(Directory),
    File(File),
}

struct Directory {
    name: String,
    children: Vec<Entry>,
}

impl Directory {
    fn size(&self) -> usize {
        self.children.iter().map(Entry::size).sum()
    }
}

struct File {
    size: usize,
}

impl Entry {
    fn size(&self) -> usize {
        match self {
            Entry::Directory(dir) => dir.size(),
            Entry::File(File { size }) => *size,
        }
    }
}

fn process_ls_line(entry: &str) -> Entry {
    let (sizedir, name) = entry.split_once(' ').unwrap();
    if sizedir == "dir" {
        Entry::Directory(Directory {
            name: name.to_owned(),
            children: Vec::new(),
        })
    } else {
        Entry::File(File {
            size: sizedir.parse().unwrap(),
        })
    }
}

fn process_commands<'a, L>(lines: &mut L, current: &mut Directory)
where
    L: Iterator<Item = &'a str>,
{
    let mut reading_ls = false;
    while let Some(line) = lines.next() {
        if line.starts_with('$') {
            let mut parts = line.split(' ').skip(1);
            match (parts.next(), parts.next()) {
                (Some("ls"), None) => {
                    reading_ls = true;
                }
                (Some("cd"), Some("..")) => {
                    break;
                }
                (Some("cd"), Some(directory)) => {
                    for entry in current.children.iter_mut() {
                        if let Entry::Directory(dir) = entry {
                            if dir.name == directory {
                                process_commands(lines, dir);
                                break;
                            }
                        }
                    }
                }
                _ => panic!("Invalid command!"),
            }
        } else {
            if !reading_ls {
                panic!("Found entry without ls");
            }
            // Entry from 'ls'
            current.children.push(process_ls_line(line));
        }
    }
}

fn keep_only_small(size: usize) -> usize {
    if size <= 100_000 {
        size
    } else {
        0
    }
}

fn sum_small_dirs(current: &Directory) -> usize {
    current
        .children
        .iter()
        .filter_map(|entry| match entry {
            Entry::Directory(dir) => Some(sum_small_dirs(dir)),
            Entry::File(_) => None,
        })
        .sum::<usize>()
        + keep_only_small(current.size())
}

fn find_possible_directories(current: &Directory, target: usize, possibilities: &mut Vec<usize>) {
    let size = current.size();
    if size > target {
        possibilities.push(size);

        // Only recurse if the current directory is at least large enough
        for entry in &current.children {
            if let Entry::Directory(dir) = entry {
                find_possible_directories(dir, target, possibilities);
            }
        }
    }
}

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> usize {
    let mut root = Directory {
        name: "/".into(),
        children: Vec::new(),
    };
    let mut lines = input.lines();
    process_commands(&mut lines, &mut root);

    sum_small_dirs(&root)
}

#[aoc(day7, part2)]
fn solve_part2(input: &str) -> usize {
    let mut root = Directory {
        name: "/".into(),
        children: Vec::new(),
    };
    let mut lines = input.lines();
    process_commands(&mut lines, &mut root);

    let unused_space = 70_000_000 - root.size();
    let target = 30_000_000 - unused_space;

    let mut possibilities = Vec::new();
    find_possible_directories(&root, target, &mut possibilities);
    possibilities.sort();
    possibilities[0]
}
