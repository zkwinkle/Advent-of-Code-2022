use std::{
    ops::{Index, IndexMut},
    slice::{Chunks, ChunksMut, Iter, IterMut},
};
use thiserror::Error;

use super::position::Position;

/// 2D rectangular grid structure

#[derive(Debug, Clone)]
pub struct Grid<T> {
    elements: Vec<T>,
    rows: usize,
    columns: usize,
}

#[derive(Error, Debug)]
#[error("vec given was of size {0} which isn't wholly divisble by the width given {1} ({0}/{1}={2:.1})", .vec_length, .width_given, (*.vec_length as f64) / (*.width_given as f64))]
pub struct WrongDimensionsError {
    vec_length: usize,
    width_given: usize,
}

fn xy2i(columns: usize, x: usize, y: usize) -> usize { y * columns + x }

impl<T> Grid<T> {
    pub fn from_vec(
        vec: Vec<T>,
        columns: usize,
    ) -> Result<Grid<T>, WrongDimensionsError> {
        if (vec.len() % columns) == 0 {
            Ok(Grid {
                rows: vec.len() / columns,
                elements: vec,
                columns,
            })
        } else {
            Err(WrongDimensionsError {
                width_given: columns,
                vec_length: vec.len(),
            })
        }
    }

    pub fn rows(&self) -> usize { self.rows }

    pub fn columns(&self) -> usize { self.columns }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.elements.get(xy2i(self.columns, x, y))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.elements.get_mut(xy2i(self.columns, x, y))
    }

    pub fn parse_grid_with<'a, I>(
        input: &'a str,
        map_fn: impl Fn(&'a str) -> I,
    ) -> Grid<T>
    where
        I: Iterator<Item = T>,
    {
        let lines = input.lines();
        let columns = lines.clone().next().unwrap().chars().count();
        let rows = lines.clone().count();

        let mut elements: Vec<T> = Vec::with_capacity(rows * columns);

        map_fn(input).for_each(|n| elements.push(n));

        Grid {
            elements,
            rows,
            columns,
        }
    }

    pub fn iter_all(&self) -> Iter<'_, T> { self.elements.iter() }
    pub fn iter_mut_all(&mut self) -> IterMut<'_, T> {
        self.elements.iter_mut()
    }

    pub fn iter_rows(&self) -> Chunks<'_, T> {
        self.elements.chunks(self.columns)
    }

    pub fn iter_mut_rows(&mut self) -> ChunksMut<'_, T> {
        self.elements.chunks_mut(self.rows)
    }
}

impl<T: Clone> Grid<T> {
    pub fn fill(&mut self, value: T) { self.elements.fill(value); }

    pub fn with_val(val: T, rows: usize, columns: usize) -> Grid<T> {
        Grid {
            elements: vec![val; rows * columns],
            columns,
            rows,
        }
    }
}

impl<T: Default> Grid<T> {
    pub fn new(rows: usize, columns: usize) -> Grid<T> {
        let mut elements = Vec::new();
        elements.resize_with(rows * columns, Default::default);
        Grid {
            elements,
            columns,
            rows,
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, coords: (usize, usize)) -> &Self::Output {
        &self.elements[xy2i(self.columns, coords.0, coords.1)]
    }
}

impl<T> Index<Position<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Position<usize>) -> &Self::Output {
        &self.elements[xy2i(self.columns, pos.x, pos.y)]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        let row_start = row * self.columns;
        &self.elements[row_start..row_start + (self.columns - 1)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut Self::Output {
        &mut self.elements[xy2i(self.columns, coords.0, coords.1)]
    }
}

impl<T> IndexMut<Position<usize>> for Grid<T> {
    fn index_mut(&mut self, pos: Position<usize>) -> &mut Self::Output {
        &mut self.elements[xy2i(self.columns, pos.x, pos.y)]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let row_start = row * self.columns;
        &mut self.elements[row_start..row_start + (self.columns - 1)]
    }
}
