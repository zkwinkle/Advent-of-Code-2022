use aoc_lib::{collections::grid::Grid, tooling::SolutionResult};

type Num = u32;

#[allow(dead_code)]
mod a_star {
    use std::{
        cmp::{Ordering, Reverse},
        collections::{BinaryHeap, HashMap},
        hash::{Hash, Hasher},
    };

    use aoc_lib::collections::grid::Grid;

    use super::Num;

    #[derive(Clone, Copy, Debug)]
    pub struct Node {
        f_score: Num,
        x: usize,
        y: usize,
        elevation: Num,
    }

    impl Node {
        pub fn new(x: usize, y: usize, elevation: char) -> Node {
            Node {
                x,
                y,
                elevation: elevation as Num,
                f_score: Num::MAX,
            }
        }

        // assumes other is neighbor
        fn distance(&self, other: &Self) -> Num {
            if !(self.x <= other.x + 1 && other.x <= self.x + 1) {
                return Num::MAX;
            }
            if !(self.y <= other.y + 1 && other.y <= self.y + 1) {
                return Num::MAX;
            }
            if !(self.elevation + 1 >= other.elevation) {
                return Num::MAX;
            }
            1
        }
    }

    type Map<T> = HashMap<Node, T>;
    type Set = BinaryHeap<Reverse<Node>>;

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

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            if self == other {
                Ordering::Equal
            } else {
                self.f_score.cmp(&other.f_score)
            }
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    fn reconstruct_path(came_from: Map<Node>, mut current: Node) -> Vec<Node> {
        let mut total_path = vec![current];
        while came_from.contains_key(&current) {
            current = *came_from.get(&current).unwrap();
            total_path.push(current)
        }

        total_path.reverse();
        total_path
    }

    fn lowest_node(set: &Set, scores: &Map<Num>) -> Node {
        let mut min = Num::MAX;
        set.iter()
            .reduce(move |min_node, n| {
                let node_score = scores.get(&n.0).unwrap_or(&Num::MAX);
                if node_score < &min {
                    min = *node_score;
                    n
                } else {
                    min_node
                }
            })
            .unwrap()
            .0
    }

    fn get_neighbors(
        grid: &Grid<Node>,
        origin: Node,
    ) -> impl Iterator<Item = Node> + '_ {
        [
            if origin.x != 0 { grid.get(origin.x - 1, origin.y) } else { None },
            grid.get(origin.x + 1, origin.y),
            if origin.y != 0 { grid.get(origin.x, origin.y - 1) } else { None },
            grid.get(origin.x, origin.y + 1),
        ]
        .into_iter()
        .filter_map(|n| n.cloned())
    }

    pub(super) fn a_star(
        mut grid: Grid<Node>,
        start: Node,
        goal: Node,
        h: impl Fn(Node) -> Num,
    ) -> Vec<Node> {
        let start = grid.get_mut(start.x, start.y).unwrap();
        start.f_score = h(*start);

        let mut open_set: Set = [Reverse(start.clone())].into();
        let mut came_from: Map<Node> = Map::new();

        let mut g_score: Map<Num> = Map::new();
        g_score.insert(start.clone(), 0);
        //let mut f_score: Map<Num> = Map::new();
        //f_score.insert(start, h(start));

        while open_set.len() != 0 {
            let current = open_set.pop().unwrap().0; //lowest_node(&open_set, &f_score);
            if current.x == goal.x && current.y == goal.y {
                return reconstruct_path(came_from, current);
            }

            //open_set.remove(&current);

            for mut neighbor in get_neighbors(&grid, current) {
                let tentative_g_score = g_score
                    .get(&current)
                    .unwrap()
                    .saturating_add(current.distance(&neighbor));

                //println!("Neighbor {neighbor:?}: {tentative_g_score}");
                if tentative_g_score
                    < *g_score.get(&neighbor).unwrap_or(&Num::MAX)
                {
                    //f_score.insert(neighbor, tentative_g_score + h(neighbor));
                    neighbor.f_score = tentative_g_score + h(neighbor);
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);

                    if !open_set
                        .iter()
                        .any(|n| n.0.x == neighbor.x && n.0.y == neighbor.y)
                    {
                        open_set.push(Reverse(neighbor));
                    }
                }
            }
        }

        panic!("Open set is empty but goal was never reached");
    }
}

mod bfs {
    use std::collections::VecDeque;

    use aoc_lib::collections::grid::Grid;

    use super::Num;

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

    fn get_adjacent(
        grid: &Grid<Node>,
        origin: Node,
    ) -> impl Iterator<Item = Node> {
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
        .filter_map(move |n| {
            if let Some(n) = n {
                if origin.elevation + 1 >= n.elevation {
                    Some(n)
                } else {
                    None
                }
            } else {
                None
            }
        })
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

        let mut q: VecDeque<Node> = [root.clone()].into();
        while let Some(current) = q.pop_front() {
            if current == goal {
                return reconstruct_path(&grid, current);
            }

            for neighbor in get_adjacent(&grid, current) {
                if let None = neighbor.parent {
                    let neighbor =
                        grid.get_mut(neighbor.x, neighbor.y).unwrap();
                    neighbor.parent = Some((current.x, current.y));
                    q.push_back(*neighbor);
                }
            }
        }

        panic!("No more adjacent nodes but goal was never reached");
    }
}

use bfs::Node;

fn input2nodes<'s>(input: &'s str) -> impl Iterator<Item = Node> + 's {
    input.lines().enumerate().flat_map(|(y, l)| {
        l.chars().enumerate().map(move |(x, c)| {
            let elevation = match c {
                'S' => 'a',
                'E' => 'z',
                c @ 'a'..='z' => c,
                _ => panic!("Unexpected char while parsing input"),
            };
            bfs::Node::new(x, y, elevation)
        })
    })
}

fn find_start<'s>(input: &'s str) -> Node {
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

fn find_end<'s>(input: &'s str) -> Node {
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

    let path = bfs::bfs(grid, start, end);

    //println!("Path: {path:?}");

    SolutionResult::Unsigned(path.len() - 1)
}

pub fn task2(input: &str) -> SolutionResult { SolutionResult::Unsigned(0) }
