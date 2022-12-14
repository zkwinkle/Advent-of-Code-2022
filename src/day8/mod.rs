use itertools::Itertools;

use crate::tooling::SolutionResult;

fn xy2i(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn parse_forest(input: &str) -> (Vec<u8>, usize) {
    let lines = input.lines();
    let width = lines.clone().next().unwrap().chars().count();
    let length = lines.clone().count();

    let mut forest: Vec<u8> = Vec::with_capacity(length * width);

    lines
        .flat_map(|l| l.chars())
        .map(|n| n.to_digit(10).unwrap() as u8)
        .for_each(|n| forest.push(n));

    (forest, width)
}

fn up_left_visibility(forest: &Vec<u8>, width: usize) -> (Vec<u8>, Vec<u8>) {
    let size = forest.len();
    let length = size / width;

    let mut up_highest: Vec<u8> = Vec::with_capacity(size);
    let mut left_highest: Vec<u8> = Vec::with_capacity(size);

    for y in 0..length {
        for x in 0..width {
            let i = xy2i(x, y, width);
            let tree = forest[i];

            if x == 0 || tree > left_highest[i - 1] {
                left_highest.push(tree);
            } else {
                left_highest.push(left_highest[i - 1]);
            }

            if y == 0 || tree > up_highest[i - width] {
                up_highest.push(tree);
            } else {
                up_highest.push(up_highest[i - width]);
            }
        }
    }

    (up_highest, left_highest)
}

fn check_visibility(
    tree: u8,
    i: usize,
    x: usize,
    y: usize,
    width: usize,
    length: usize,
    u: &Vec<u8>,
    d: &Vec<u8>,
    l: &Vec<u8>,
    r: &Vec<u8>,
) -> bool {
    if x == 0 || x == width - 1 || y == 0 || y == length - 1 {
        true
    } else {
        tree > l[i - 1] || tree > r[i + 1] || tree > u[i - width] || tree > d[i + width]
    }
}

pub fn task1(input: &str) -> SolutionResult {
    let (mut forest, width) = parse_forest(input);

    let size = forest.len();
    let length = size / width;

    let (up, left) = up_left_visibility(&forest, width);
    forest.reverse();
    let (mut down, mut right) = up_left_visibility(&forest, width);
    down.reverse();
    right.reverse();
    forest.reverse();

    let res = (0..length)
        .cartesian_product(0..width)
        .zip(forest)
        .enumerate()
        //.inspect(|&(i, ((y,x), t))| println!("({x},{y}): {t} (#{i})"))
        .filter(|&(i, ((y,x), t))| {
            check_visibility(t, i, x, y, width, length, &up, &down, &left, &right)
        })
        //.inspect(|(i, t)| {
        //    let (x, y) = i2xy(*i, width);
        //    println!("Tree visible at ({x}, {y}): {t}");
        //})
        .count();

    SolutionResult::Unsigned(res)
}

fn view_up(forest: &Vec<u8>, width: usize, mut i: usize) -> usize {
    let tree = forest[i];
    let mut v = 0;
    while i >= width {
        v += 1;
        i -= width;
        if forest[i] >= tree {
            break;
        }
    }

    v
}

fn view_left(forest: &Vec<u8>, x: usize, i: usize) -> usize {
    let tree = forest[i];
    let mut v = 0;

    for i in (i - x..i).rev() {
        v += 1;
        if forest[i] >= tree {
            break;
        }
    }

    v
}

fn up_left_views(forest: &Vec<u8>, width: usize) -> (Vec<usize>, Vec<usize>) {
    let size = forest.len();
    let length = size / width;

    let mut up_views: Vec<usize> = Vec::with_capacity(size);
    let mut left_views: Vec<usize> = Vec::with_capacity(size);

    for y in 0..length {
        for x in 0..width {
            let i = xy2i(x, y, width);
            let tree = forest[i];

            if x == 0 {
                left_views.push(0);
            } else if forest[i - 1] >= tree {
                left_views.push(1);
            } else {
                left_views.push(view_left(forest, x, i));
            }

            if y == 0 {
                up_views.push(0);
            } else if forest[i - width] >= tree {
                up_views.push(1);
            } else {
                up_views.push(view_up(forest, width, i));
            }
        }
    }

    (up_views, left_views)
}

pub fn task2(input: &str) -> SolutionResult {
    let (mut forest, width) = parse_forest(input);

    let size = forest.len();
    let (up_views, left_views) = up_left_views(&forest, width);
    forest.reverse();
    let (mut down_views, mut right_views) = up_left_views(&forest, width);
    down_views.reverse();
    right_views.reverse();

    let res = (0..size)
        .map(|i| up_views[i] * down_views[i] * left_views[i] * right_views[i])
        .max()
        .unwrap();

    SolutionResult::Unsigned(res)
}
