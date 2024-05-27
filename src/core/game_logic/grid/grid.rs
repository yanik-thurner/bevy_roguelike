use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut, RangeInclusive};

use encoding::*;

use super::prelude::*;

#[derive(Clone, Eq, PartialEq)]
pub struct Grid<T, E>

{
    positions: Vec<Position>,
    cells: Vec<T>,
    dimensions: (usize, usize),
    offset: (i32, i32),
    encoding: E,
}

impl<T, E> Grid<T, E>
    where T: Clone + Debug,
          E: Encoding2D
{
    fn create(dimensions: (usize, usize), offset: (i32, i32), default: T) -> Self {
        let capacity = dimensions.0 * dimensions.1;
        let encoding = E::new(dimensions, offset);
        let positions = (0..capacity).map(|i| Position::from_tuple(encoding.decode(i))).collect();

        Grid {
            positions,
            cells: vec![default; capacity],
            dimensions,
            offset,
            encoding,
        }
    }

    pub fn new(dimensions: (usize, usize), offset: (i32, i32), default: T) -> Self {
        Self::create(dimensions, offset, default)
    }

    pub fn new_with_minmax(min_xy: Position, max_xy: Position, default: T) -> Self {
        let width = 1 + (max_xy.x - min_xy.x) as usize;
        let height = 1 + (max_xy.y - min_xy.y) as usize;
        let dimensions = (width, height);
        let offset = min_xy.to_tuple();
        Self::create(dimensions, offset, default)
    }

    pub fn map<U: Clone + Debug, F: Encoding2D>(&self, mapping_function: fn(&T) -> U) -> Grid<U, F> {
        let capacity = self.dimensions.0 * self.dimensions.1;
        let encoding = F::new(self.dimensions, self.offset);

        let positions: Vec<Position> = (0..capacity).map(|i| Position::from_tuple(encoding.decode(i))).collect();
        let cells = positions.iter().map(|pos| mapping_function(&self[pos])).collect();
        Grid {
            positions,
            cells,
            dimensions: self.dimensions,
            offset: self.offset,
            encoding,
        }
    }

    pub fn try_get(&self, position: &Position) -> Option<&T> {
        let idx = self.encoding.encode(position.x, position.y);
        if self.contains_position(position) {
            Some(&self.cells[idx])
        } else {
            None
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    pub fn get_min(&self) -> Position {
        Position::from_tuple(self.offset)
    }

    pub fn get_max(&self) -> Position {
        Position::new(self.dimensions.0 as i32 + self.offset.0 - 1, self.dimensions.1 as i32 + self.offset.1 - 1)
    }

    pub fn x_range(&self) -> RangeInclusive<i32> {
        self.get_min().x..=self.get_max().x
    }

    pub fn y_range(&self) -> RangeInclusive<i32> {
        self.get_min().y..=self.get_max().y
    }

    pub fn contains_position(&self, position: &Position) -> bool {
        position.x >= self.get_min().x && position.x <= self.get_max().x
            && position.y >= self.get_min().y && position.y <= self.get_max().y
    }


    pub fn iter_cells(&self) -> GridIterator<'_, T, E> {
        GridIterator {
            grid: self,
            index: 0,
        }
    }

    pub fn iter_cells_mut(&mut self) -> GridIteratorMut<'_, T, E> {
        GridIteratorMut {
            grid: self,
            index: 0,
        }
    }
}


impl<T, E: Encoding2D> Index<&Position> for Grid<T, E> {
    type Output = T;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.cells[self.encoding.encode(index.x, index.y)]
    }
}

impl<T, E: Encoding2D> IndexMut<&Position> for Grid<T, E> {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        &mut self.cells[self.encoding.encode(index.x, index.y)]
    }
}

pub struct GridIterator<'a, T, E> {
    grid: &'a Grid<T, E>,
    index: usize,
}

impl<'a, T, E: Encoding2D> Iterator for GridIterator<'a, T, E> {
    type Item = (&'a Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.grid.cells.len() {
            let cell = &self.grid.cells[self.index];
            let position = &self.grid.positions[self.index];
            self.index += 1;
            Some((position, cell))
        } else {
            None
        }
    }
}

pub struct GridIteratorMut<'a, T, E> {
    grid: &'a mut Grid<T, E>,
    index: usize,
}

impl<'a, T, E: Encoding2D> Iterator for GridIteratorMut<'a, T, E> {
    type Item = (&'a Position, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.grid.cells.len() {
            let cell_ptr = self.grid.cells.as_mut_ptr();
            let cell = unsafe { &mut *cell_ptr.add(self.index) };
            let position_ptr = self.grid.positions.as_mut_ptr();
            let position = unsafe { &*position_ptr.add(self.index) };
            self.index += 1;
            Some((position, cell))
        } else {
            None
        }
    }
}

impl<T: Clone + Debug, E: Encoding2D> Debug for Grid<T, E> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        let element_length = self.cells.iter().map(|cell| format!("{:?}", cell).len()).max().unwrap();

        println!("dimensions (w, h): ({}, {}); offset (x, y): ({}, {});",
                 self.dimensions.0, self.dimensions.1,
                 self.offset.0, self.offset.1);

        println!("  |{}|", "_".repeat((element_length + 3) * self.dimensions.0 - 1));
        print!("  ", );
        self.x_range().into_iter().for_each(|x| print!("| {:02}{}", x, " ".repeat(element_length - 1)));
        println!();
        println!("  |{}|", "_".repeat((element_length + 3) * self.dimensions.0 - 1));
        for y in self.y_range().rev() {
            print!("{:02}|", y);
            for x in self.x_range() {
                print!(" ");
                print!("{:?}", self[&Position::new(x, y)]);
                print!(" |");
            }
            println!("{:02}", y);
        }
        println!("  |{}|", "_".repeat((element_length + 3) * self.dimensions.0 - 1));
        print!("  ", );
        self.x_range().into_iter().for_each(|x| print!("| {:02}{}", x, " ".repeat(element_length - 1)));
        println!();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Display};

    use crate::core::game_logic::grid::grid::encoding::{Encoding2D, YFirstEncoding, ZCurveEncoding};
    use crate::core::game_logic::grid::grid::Grid;
    use crate::prelude::Position;

    fn test_set_value<T: Debug + Clone, E: Encoding2D>(grids: Vec<Grid<usize, E>>) {
        for mut grid in grids {
            let min_xy = Position::new(grid.offset.0, grid.offset.1);
            let max_xy = Position::new(grid.offset.0 + grid.dimensions.0 as i32 - 1,
                                       grid.offset.1 + grid.dimensions.1 as i32 - 1);
            assert!(grid.contains_xy(min_xy.x, min_xy.y));
            assert!(grid.contains_xy(max_xy.x, min_xy.y));
            assert!(grid.contains_xy(min_xy.x, max_xy.y));
            assert!(grid.contains_xy(max_xy.x, max_xy.y));

            println!("{:?}", grid);
            for y in grid.y_range() {
                for x in grid.x_range() {
                    let idx = grid.encoding.encode(x, y);
                    println!("{} {} => {}", x, y, idx);
                    grid[(x, y)] = idx;
                    assert_eq!(idx, grid[(x, y)]);
                    assert_eq!((x, y), grid.encoding.decode(idx));
                }
            }
            println!("{:?}", grid);
        }
    }

    #[test]
    fn test_set_value_yfirst() {
        let grids = vec![
            Grid::<usize, YFirstEncoding>::new((10, 10), (0, 0), 0),
            Grid::<usize, YFirstEncoding>::new((10, 10), (-5, -5), 0),
            Grid::<usize, YFirstEncoding>::new((10, 10), (10, 20), 0),
            Grid::<usize, YFirstEncoding>::new((20, 10), (100, 200), 0),
        ];
        test_set_value::<usize, _>(grids);
    }

    #[test]
    fn test_set_value_zcurve() {
        println!("{:?}", (1, 2));
        let grids = vec![
            Grid::<usize, ZCurveEncoding>::new((8, 8), (0, 0), 0),
            Grid::<usize, ZCurveEncoding>::new((16, 16), (0, 0), 0),
            Grid::<usize, ZCurveEncoding>::new((32, 32), (-16, -16), 0),
            Grid::<usize, ZCurveEncoding>::new((32, 32), (17, -18), 0),
        ];
        test_set_value::<usize, _>(grids);
    }
}