use std::cmp::Ordering;

use aoc_lib::{
    structs::{grid::Grid, position::Position},
    tooling::SolutionResult,
};

const MIN_X: usize = 300;
const MAX_X: usize = 700;
const MIN_Y: usize = 0;
const MAX_Y: usize = 200;
// For drawing on testinput
//const MIN_X: usize = 480;
//const MAX_X: usize = 520;
//const MIN_Y: usize = 0;
//const MAX_Y: usize = 12;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    Sand,
    Air,
    Wall,
}

impl Default for Tile {
    fn default() -> Self { Tile::Air }
}

/// Spawns a grain of sand and modifies the grid changing an Air tile with a
/// Sand tile in its final resting position, and returns said Position. If the
/// Sand flows off to the abyss then the grid is left untouched and the position
/// will have the last position of the sand before it left the grid.
fn spawn_sand(
    grid: &mut Grid<Tile>,
    spawn_position: Position<usize>,
) -> Position<usize> {
    if spawn_position.x >= grid.columns() || spawn_position.y >= grid.rows() {
        panic!("Spawn position out of bounds, the dimensions are {:?} but the spawn is {spawn_position}", (grid.columns(), grid.rows()) );
    }
    if grid[spawn_position] != Tile::Air {
        panic!("Attempted to spawn sand at a clogged spawn position");
    }

    let mut current_pos = spawn_position;

    loop {
        let down_left = grid.get(
            current_pos
                .x
                .checked_sub(1)
                .unwrap_or_else(|| grid.columns()),
            current_pos.y + 1,
        );
        let down_center = grid.get(current_pos.x, current_pos.y + 1);
        let down_right = grid.get(current_pos.x + 1, current_pos.y + 1);

        //println!("Current: {current_pos}, DL: {down_left:?}, DC: {down_center:?}, DR: {down_right:?}");

        match (down_left, down_center, down_right) {
            // Move down straight
            (_, Some(Tile::Air), _) => {
                while let Some(Tile::Air) =
                    grid.get(current_pos.x, current_pos.y + 1)
                {
                    //println!("Freefalling! {current_pos}");
                    current_pos.y += 1;
                }
            }
            // Move down left
            (Some(Tile::Air), Some(Tile::Wall | Tile::Sand), _) => {
                current_pos.x -= 1;
                current_pos.y += 1;
            }
            // Move down right
            (
                Some(Tile::Wall | Tile::Sand),
                Some(Tile::Wall | Tile::Sand),
                Some(Tile::Air),
            ) => {
                current_pos.x += 1;
                current_pos.y += 1;
            }
            // Blocked, stop
            (
                Some(Tile::Wall | Tile::Sand),
                Some(Tile::Wall | Tile::Sand),
                Some(Tile::Wall | Tile::Sand),
            ) => {
                grid[current_pos] = Tile::Sand;
                break;
            }
            // Reached bottom, stop
            (_, None, _) => {
                break;
            }
            // Overflow to left, stop at bottom left corner
            (None, Some(Tile::Wall | Tile::Sand), _) => {
                current_pos = Position::new(0, grid.rows() - 1);
                break;
            }
            // Overflow to right, stop at bottom right corner
            (
                Some(Tile::Wall | Tile::Sand),
                Some(Tile::Wall | Tile::Sand),
                None,
            ) => {
                current_pos =
                    Position::new(grid.columns() - 1, grid.rows() - 1);
                break;
            }
        }
    }

    current_pos
}

fn parse(input: &str) -> Grid<Tile> {
    let coords_iter = input.lines().map(|l| {
        l.split(" -> ")
            .map(|coords| {
                let coords = coords.split_once(',').unwrap();
                Position::new(
                    coords.0.parse::<usize>().unwrap() - MIN_X,
                    coords.1.parse::<usize>().unwrap() - MIN_Y,
                )
            })
            .peekable()
    });

    let mut grid = Grid::new(MAX_Y - MIN_Y, MAX_X - MIN_X);

    for mut wall in coords_iter {
        let mut current = wall.next().unwrap();
        while let Some(next) = wall.peek() {
            match (next.x.cmp(&current.x), next.y.cmp(&current.y)) {
                (Ordering::Greater, Ordering::Equal) => {
                    for x in current.x..=next.x {
                        grid[current.y][x] = Tile::Wall;
                    }
                }
                (Ordering::Equal, Ordering::Greater) => {
                    for y in current.y..=next.y {
                        grid[y][current.x] = Tile::Wall;
                    }
                }
                (Ordering::Less, Ordering::Equal) => {
                    for x in next.x..=current.x {
                        grid[current.y][x] = Tile::Wall;
                    }
                }
                (Ordering::Equal, Ordering::Less) => {
                    for y in next.y..=current.y {
                        grid[y][current.x] = Tile::Wall;
                    }
                }
                (Ordering::Equal, Ordering::Equal) => {
                    panic!("false assumption (non-repeat vertices)")
                }
                (
                    Ordering::Greater | Ordering::Less,
                    Ordering::Greater | Ordering::Less,
                ) => {
                    panic!("false assumption (straight walls)")
                }
            }
            current = wall.next().unwrap();
        }
    }

    grid
}

#[allow(dead_code)]
fn draw_grid(grid: &Grid<Tile>) {
    for row in grid.iter_rows() {
        for tile in row {
            print!(
                "{}",
                match tile {
                    Tile::Air => ' ',
                    Tile::Sand => '𐬽',
                    Tile::Wall => '#',
                }
            )
        }
        println!();
    }
}

pub fn task1(input: &str) -> SolutionResult {
    let mut grid: Grid<Tile> = parse(input);

    let spawn_position = Position::new(500, 0) - Position::new(MIN_X, MIN_Y);

    let mut counter = 0;
    let mut resting_position = spawn_sand(&mut grid, spawn_position);

    while resting_position.y + 1 < MAX_Y {
        //println!("Sand finished on {resting_position}");
        counter += 1;
        resting_position = spawn_sand(&mut grid, spawn_position);
    }

    //draw_grid(&grid);

    SolutionResult::Unsigned(counter)
}

fn fill_pyramid(
    grid: &mut Grid<Tile>,
    spawn_position: Position<usize>,
    floor_y: usize,
) -> usize {
    let mut count = 1;
    grid[spawn_position] = Tile::Sand;
    let mid = spawn_position.x;

    for row in (spawn_position.y + 1)..floor_y {
        let width: usize = (row + 1) * 2 - 1;
        let offset: usize = width / 2;
        for col in (mid - offset)..=(mid + offset) {
            match (
                grid.get(col - 1, row - 1),
                grid.get(col, row - 1),
                grid.get(col + 1, row - 1),
            ) {
                (Some(Tile::Sand), ..)
                | (_, Some(Tile::Sand), _)
                | (_, _, Some(Tile::Sand)) => {
                    if grid[row][col] == Tile::Air {
                        grid[row][col] = Tile::Sand;
                        count += 1;
                    }
                }
                _ => (),
            }
        }
    }
    count
}

pub fn task2(input: &str) -> SolutionResult {
    let mut grid: Grid<Tile> = parse(input);

    let max_y = grid.iter_rows().enumerate().fold(0, |max_y, (i, row)| {
        if row.contains(&Tile::Wall) {
            i
        } else {
            max_y
        }
    });

    if grid.columns() <= ((max_y + 2) * 2 - 1) {
        panic!("Not enough columns for correct simulation of part2, current columns are {} but need {}", grid.columns(), ((max_y+2)*2 - 1))
    }

    // Unnecessary, uncomment if you wanna draw floor
    //let floor = &mut grid[max_y + 2];
    //floor.fill(Tile::Wall);

    let spawn_position = Position::new(500, 0) - Position::new(MIN_X, MIN_Y);

    let counter = fill_pyramid(&mut grid, spawn_position, max_y + 2);

    //draw_grid(&grid);

    SolutionResult::Unsigned(counter)
}
