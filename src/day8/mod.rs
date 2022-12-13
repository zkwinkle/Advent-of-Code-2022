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

fn sum_view<'a, T: Iterator<Item = &'a u8>>(mut iter: T) -> usize {
    let tree = iter.next().unwrap();

    let mut count = 0;
    for t in iter {
        count += 1;
        if t >= tree {
            break;
        }
    }
    count
}

fn view_along_x(vec: &Vec<u8>, width: usize, x: usize, y: usize) -> usize {
    let i = xy2i(x, y, width);
    sum_view(vec[i - x..i + 1].iter().rev()) * sum_view(vec[i..i + (width - x)].iter())
}

fn view_along_y(vec: &Vec<u8>, width: usize, x: usize, y: usize) -> usize {
    sum_view(vec.iter().skip(x).step_by(width).take(y + 1).rev())
        * sum_view(vec.iter().skip(x).step_by(width).skip(y))
}

fn get_view_score(vec: &Vec<u8>, i: usize, width: usize) -> usize {
    let (x, y) = i2xy(i, width);

    let score = view_along_x(vec, width, x, y) * view_along_y(vec, width, x, y);
    //println!("Tree {} has view score of: {score}", vec[i]);
    return score;
}

pub fn task2(input: &str) -> SolutionResult {
    let (forest, width) = parse_forest(input);

    let res = forest
        .iter()
        .enumerate()
        .map(|(i, _)| get_view_score(&forest, i, width))
        .max()
        .unwrap();
    //.inspect(|(i, t)| {
    //    let (x, y) = i2xy(*i, width);
    //    println!("Tree visible at ({x}, {y}): {t}");
    //})

    SolutionResult::Unsigned(res)
}
