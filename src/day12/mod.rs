use aoc_lib::{collections::grid::Grid, tooling::SolutionResult};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
    thread,
};

type Num = u32;

#[derive(Clone, Copy, Debug)]
struct Node {
    x: usize,
    y: usize,
    elevation: Num,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Node {}
impl Node {
    fn new(x: usize, y: usize, elevation: char) -> Node {
        Node {
            x,
            y,
            elevation: elevation as Num,
        }
    }
}

fn get_adjacent(grid: &Grid<Node>, origin: Node) -> impl Iterator<Item = Node> {
    [
        if origin.x != 0 {
            grid.get(origin.x - 1, origin.y).cloned()
        } else {
            None
        },
        grid.get(origin.x + 1, origin.y).cloned(),
        if origin.y != 0 {
            grid.get(origin.x, origin.y - 1).cloned()
        } else {
            None
        },
        grid.get(origin.x, origin.y + 1).cloned(),
    ]
    .into_iter()
    .filter_map(move |n| n.filter(|&n| origin.elevation + 1 >= n.elevation))
}

fn count_path(came_from: FxHashMap<Node, Node>, end: Node) -> usize {
    let mut current = end;
    let mut count = 0;
    while let Some(parent) = came_from.get(&current) {
        current = *parent;
        count += 1;
    }

    count
}

fn bfs(grid: &Grid<Node>, root: Node, goal: Node) -> usize {
    let mut q: VecDeque<Node> = VecDeque::with_capacity(100);
    q.push_back(root);
    let mut explored_nodes: FxHashSet<Node> =
        HashSet::with_capacity_and_hasher(100, Default::default());
    explored_nodes.insert(root);
    let mut came_from: FxHashMap<Node, Node> =
        HashMap::with_capacity_and_hasher(100, Default::default());

    while let Some(current) = q.pop_front() {
        if current == goal {
            return count_path(came_from, current);
        }

        for neighbor in get_adjacent(&grid, current) {
            if !explored_nodes.contains(&neighbor) {
                q.push_back(neighbor);
                explored_nodes.insert(neighbor);
                came_from.insert(neighbor, current);
            }
        }
    }

    // No more adjacent nodes but goal was never reached
    usize::MAX
}

fn input2nodes(input: &str) -> impl Iterator<Item = Node> + '_ {
    input.lines().enumerate().flat_map(|(y, l)| {
        l.chars().enumerate().map(move |(x, c)| {
            let elevation = match c {
                'S' => 'a',
                'E' => 'z',
                c @ 'a'..='z' => c,
                _ => panic!("Unexpected char while parsing input"),
            };
            Node::new(x, y, elevation)
        })
    })
}

fn find_start(input: &str) -> Node {
    input
        .lines()
        .enumerate()
        .find_map(|(y, l)| {
            l.chars().enumerate().find_map(move |(x, c)| {
                if c == 'S' {
                    Some(Node::new(x, y, 'a'))
                } else {
                    None
                }
            })
        })
        .unwrap()
}

fn find_end(input: &str) -> Node {
    input
        .lines()
        .enumerate()
        .find_map(|(y, l)| {
            l.chars().enumerate().find_map(move |(x, c)| {
                if c == 'E' {
                    Some(Node::new(x, y, 'z'))
                } else {
                    None
                }
            })
        })
        .unwrap()
}

pub fn task1(input: &str) -> SolutionResult {
    let grid: Grid<Node> = Grid::parse_grid(input, input2nodes);
    let start = find_start(input);
    let end = find_end(input);

    let path = bfs(&grid, start, end);

    SolutionResult::Unsigned(path)
}

fn find_starts(input: &str) -> impl Iterator<Item = Node> + '_ {
    input
        .lines()
        .enumerate()
        .filter_map(|(y, l)| {
            Some(l.chars().enumerate().filter_map(move |(x, c)| {
                if c == 'S' || c == 'a' {
                    Some(Node::new(x, y, 'a'))
                } else {
                    None
                }
            }))
        })
        .flatten()
}

pub fn task2(input: &str) -> SolutionResult {
    let grid: Grid<Node> = Grid::parse_grid(input, input2nodes);
    let starts = find_starts(input);
    let end = find_end(input);
    let mut thread_count = 0;

    let shortest_path = thread::scope(|scope| {
        let grid = &grid;
        let mut threads = Vec::with_capacity(50);
        for start in starts {
            thread_count += 1;
            threads.push(scope.spawn(move || bfs(grid, start, end)));
        }

        threads
            .into_iter()
            .map(|thread| thread.join().unwrap())
            .min()
            .unwrap()
    });

    SolutionResult::Unsigned(shortest_path)
}
