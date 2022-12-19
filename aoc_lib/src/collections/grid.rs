use std::ops::Index;

/// 2D rectangular grid structure

#[derive(Debug, Clone)]
pub struct Grid<T> {
    elements: Vec<T>,
    rows: usize,
    columns: usize,
}

fn xy2i(columns: usize, x: usize, y: usize) -> usize { y * columns + x }

impl<T> Grid<T> {
    pub fn rows(&self) -> usize { self.rows }

    pub fn columns(&self) -> usize { self.columns }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.elements.get(xy2i(self.columns, x, y))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.elements.get_mut(xy2i(self.columns, x, y))
    }

    pub fn parse_grid<'a, I>(
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
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, coords: (usize, usize)) -> &Self::Output {
        &self.elements[xy2i(self.columns, coords.0, coords.1)]
    }
}
