use crate::tooling::SolutionResult;

fn xy2i(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}
fn i2xy(i: usize, width: usize) -> (usize, usize) {
    (i % width, i / width)
}

fn check_along_x(vec: &Vec<u8>, width: usize, x: usize, y: usize) -> bool {
    let tree = vec[xy2i(x, y, width)];
    let i = xy2i(x, y, width);
    !(vec[i - x..i].iter().any(|&t| t >= tree)
        && vec[i + 1..i + (width - x)].iter().any(|&t| t >= tree))
}

fn check_along_y(vec: &Vec<u8>, width: usize, x: usize, y: usize) -> bool {
    let tree = vec[xy2i(x, y, width)];
    !(vec
        .iter()
        .skip(x)
        .step_by(width)
        .take(y)
        //.inspect(|t| {
        //    println!("Inspecting tree: {t}");
        //})
        .any(|&t| t >= tree)
        && vec
            .iter()
            .skip(x)
            .step_by(width)
            .skip(y + 1)
            //.inspect(|t| {
            //    println!("Inspecting tree2: {t}");
            //})
            .any(|&t| t >= tree))
}

fn check_visibility(vec: &Vec<u8>, i: usize, width: usize) -> bool {
    let (x, y) = i2xy(i, width);

    check_along_x(vec, width, x, y) || check_along_y(vec, width, x, y)
}

pub fn parse_forest(input: &str) -> (Vec<u8>, usize) {
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

pub fn task1(input: &str) -> SolutionResult {
    let (forest, width) = parse_forest(input);

    let res = forest
        .iter()
        .enumerate()
        .filter(|(i, _)| check_visibility(&forest, *i, width))
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

fn up_left_views(forest: &Vec<u8>, width: usize, size: usize) -> (Vec<usize>, Vec<usize>) {
    let mut up_views: Vec<usize> = Vec::with_capacity(size);
    let mut left_views: Vec<usize> = Vec::with_capacity(size);

    for (i, tree) in forest.iter().enumerate() {
        let (x, y) = i2xy(i, width);

        if x == 0 {
            left_views.push(0);
        } else if forest[i - 1] >= *tree {
            left_views.push(1);
        } else {
            left_views.push(view_left(forest, x, i));
        }

        if y == 0 {
            up_views.push(0);
        } else if forest[i - width] >= *tree {
            up_views.push(1);
        } else {
            up_views.push(view_up(forest, width, i));
        }
    }

    (up_views, left_views)
}

pub fn task2(input: &str) -> SolutionResult {
    let (mut forest, width) = parse_forest(input);

    let size = forest.len();
    let (up_views, left_views) = up_left_views(&forest, width, size);
    forest.reverse();
    let (mut down_views, mut right_views) = up_left_views(&forest, width, size);
    down_views.reverse();
    right_views.reverse();

    let res = (0..size)
        .map(|i| up_views[i] * down_views[i] * left_views[i] * right_views[i])
        .max()
        .unwrap();

    SolutionResult::Unsigned(res)
}
