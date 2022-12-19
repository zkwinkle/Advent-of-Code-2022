use aoc_lib::{collections::grid::Grid, tooling::SolutionResult};

type Num = u32;

use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
pub struct Node {
    x: usize,
    y: usize,
    elevation: Num,
    parent: Option<(usize, usize)>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Node {}
impl Node {
    pub fn new(x: usize, y: usize, elevation: char) -> Node {
        Node {
            x,
            y,
            elevation: elevation as Num,
            parent: None,
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

fn reconstruct_path(grid: &Grid<Node>, end: Node) -> Vec<Node> {
    let mut current = end;
    let mut total_path = vec![current];
    while let Some(parent_xy) = current.parent {
        // Root's parent is itself
        if parent_xy.0 == current.x && parent_xy.1 == current.y {
            break;
        }
        current = *grid.get(parent_xy.0, parent_xy.1).unwrap();
        total_path.push(current)
    }

    total_path.reverse();
    total_path
}

pub fn bfs(mut grid: Grid<Node>, root: Node, goal: Node) -> Vec<Node> {
    let root = grid.get_mut(root.x, root.y).unwrap();
    root.parent = Some((root.x, root.y));

    let mut q: VecDeque<Node> = [*root].into();
    while let Some(current) = q.pop_front() {
        if current == goal {
            return reconstruct_path(&grid, current);
        }

        for neighbor in get_adjacent(&grid, current) {
            if neighbor.parent.is_none() {
                let neighbor = grid.get_mut(neighbor.x, neighbor.y).unwrap();
                neighbor.parent = Some((current.x, current.y));
                q.push_back(*neighbor);
            }
        }
    }

    panic!("No more adjacent nodes but goal was never reached");
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

    let path = bfs(grid, start, end);

    //println!("Path: {path:?}");

    SolutionResult::Unsigned(path.len() - 1)
}

pub fn task2(input: &str) -> SolutionResult { SolutionResult::Unsigned(0) }
