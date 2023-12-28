#![allow(dead_code)]

use std::fmt;
use std::ops;

// Grid by Belén Albeza
// https://github.com/belen-albeza/aoc-2021/blob/main/src/utils.rs
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    pub cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(cells: &[T], width: usize) -> Self {
        let len = cells.len();
        let cells = cells.to_owned();
        Self {
            cells,
            width,
            height: len / width,
        }
    }

    pub fn from(input: &str, mapfn: fn(cell: char) -> T) -> Grid<T> {
        let vec : Vec<Vec<T>> = input.lines().map(|s| s.chars().map(mapfn).collect()).collect();
        let width = vec.first().unwrap().len();
        let cells = vec.into_iter().flatten().collect::<Vec<T>>();
        Grid::new(&cells, width)
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn cell_at(&self, x: i32, y: i32) -> Option<T> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        let i = self.index_for(x as usize, y as usize);
        Some(self.cells[i as usize].clone())
    }

    pub fn set_at(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            panic!("Trying to set unexisting coordinates: ({}, {})", x, y);
        }

        let i = self.index_for(x, y);
        self.cells[i] = value;
    }

    pub fn neighbors_at(&self, x: i32, y: i32) -> Vec<(T, (usize, usize))> {
        [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
            .iter()
            .filter_map(|point| {
                self.cell_at(point.0, point.1)
                    .map(|cell| (cell, (point.0 as usize, point.1 as usize)))
            })
            .collect()
    }

    pub fn neighbors8_at(&self, x: i32, y: i32) -> Vec<(T, (usize, usize))> {
        [
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
        ]
        .iter()
        .filter_map(|point| {
            self.cell_at(point.0, point.1)
                .map(|cell| (cell, (point.0 as usize, point.1 as usize)))
        })
        .collect()
    }

    fn index_for(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn println(&self, map: fn(cell: &T) -> String) {
        let mut buffer = "".to_string();
        for y in 0..self.size().1 {
            for x in 0..self.size().0 {
                let cell = self.cell_at(x as i32, y as i32).unwrap();
                buffer.push_str(&map(&cell));
            }
            buffer.push('\n');
        }
        println!("{}", buffer)
    }
}

impl<T: Clone + fmt::Debug> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = "".to_string();
        for y in 0..self.size().1 {
            for x in 0..self.size().0 {
                buffer.push_str(&format!("{:?}", self.cell_at(x as i32, y as i32).unwrap()));
            }
            buffer.push('\n');
        }
        writeln!(f, "{}", buffer)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
pub type Point = Vector<i32>;

impl<T: Default> Vector<T> {
    pub fn new(x: T, y: T) -> Vector<T> {
        Vector { x, y, z: T::default() }
    }

    pub fn new_3d(x: T, y: T, z: T) -> Vector<T> {
        Vector { x, y, z }
    }
}

impl Point {
    pub fn manhattan_dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl<T: std::ops::Add<Output = T>> ops::Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, _rhs: Vector<T>) -> Vector<T> {
        Vector {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl<T: std::ops::Sub<Output = T>> ops::Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, _rhs: Vector<T>) -> Vector<T> {
        Vector {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}


impl<T: std::ops::Mul<Output = T> + Copy> ops::Mul<T> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, _rhs: T) -> Vector<T> {
        Vector {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl<T : std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy> Vector<T> {
    pub fn dot(&self, other: &Vector<T>) -> T {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
    
    pub fn length_sq(&self) -> T {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn cross(&self, other: &Vector<T>) -> Vector<T> {
        Vector {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x
        }
    }
}