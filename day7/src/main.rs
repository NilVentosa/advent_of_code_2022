use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::rc::Rc;

const FILENAME: &str = "input.txt";

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn get_input() -> BufReader<File> {
    BufReader::new(File::open(FILENAME).expect("Error opening file"))
}

#[derive(Debug)]
enum Line {
    Ls,
    Cd(String),
    Dir(String),
    File(u64, String),
}

type NodeHandle = Rc<RefCell<Node>>;

fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}

#[derive(Default, Debug)]
struct Node {
    size: usize,
    children: HashMap<String, NodeHandle>,
    parent: Option<NodeHandle>,
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn total_size(&self) -> u64 {
        self.children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size as u64
    }
}

fn get_root() -> NodeHandle {
    let lines = get_input().lines().map(|l| parse_line(l.unwrap()));

    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();

    for line in lines {
        match line {
            Line::Ls => {}
            Line::Cd(path) => match path.as_str() {
                "/" => {}
                ".." => {
                    let parent = node.borrow().parent.clone().unwrap();
                    node = parent;
                }
                _ => {
                    let child = node.borrow_mut().children.entry(path).or_default().clone();
                    node = child;
                }
            },
            Line::Dir(dir) => {
                let entry = node.borrow_mut().children.entry(dir).or_default().clone();
                entry.borrow_mut().parent = Some(node.clone());
            }
            Line::File(size, file) => {
                let entry = node.borrow_mut().children.entry(file).or_default().clone();
                entry.borrow_mut().size = size as usize;
                entry.borrow_mut().parent = Some(node.clone());
            }
        }
    }

    root
}

fn parse_line(line: String) -> Line {
    if line.starts_with("$ cd") {
        return Line::Cd(line.split(' ').last().unwrap().to_string());
    }
    if line.starts_with("$ ls") {
        return Line::Ls;
    }
    if line.starts_with("dir ") {
        return Line::Dir(line.split(' ').last().unwrap().to_string());
    }
    let thing: Vec<&str> = line.split(' ').collect();
    Line::File(
        thing.get(0).unwrap().parse::<u64>().unwrap(),
        thing.get(1).unwrap().to_string(),
    )
}

fn part_one() -> u64 {
    let root = get_root();
    let sum = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>();

    return sum;
}

fn part_two() -> u64 {
    let root = get_root();
    let total_space = 70000000_u64;
    let used_space = root.borrow().total_size();
    let free_space = total_space.checked_sub(used_space).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let removed_dir_size = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .min();

    return removed_dir_size.unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let result = super::part_one();
        assert_eq!(result, 1783610);
    }

    #[test]
    fn part_two() {
        let result = super::part_two();
        assert_eq!(result, 4370655);
    }
}
