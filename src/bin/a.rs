#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]

use proconio::input;
use rand::prelude::*;

#[derive(Default)]
struct Solver {}
impl Solver {
    fn solve(&mut self) {}
}

fn main() {
    let start = std::time::Instant::now();

    solve();

    #[allow(unused_mut, unused_assignments)]
    let mut elapsed_time = start.elapsed().as_micros() as f64 * 1e-6;
    #[cfg(feature = "local")]
    {
        eprintln!("Local Mode");
        elapsed_time *= 0.55;
    }
    eprintln!("Elapsed: {}", (elapsed_time * 1000.0) as usize);
}

fn solve() {
    let time_limit = 1.98;
    let time_keeper = TimeKeeper::new(time_limit);
}

struct Input {}

fn read_input() -> Input {
    input! {}
    Input {}
}

#[derive(Debug, Clone)]
struct TimeKeeper {
    start_time: std::time::Instant,
    time_threshold: f64,
}

impl TimeKeeper {
    fn new(time_threshold: f64) -> Self {
        TimeKeeper {
            start_time: std::time::Instant::now(),
            time_threshold,
        }
    }
    #[inline]
    fn isTimeOver(&self) -> bool {
        let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            elapsed_time * 0.55 >= self.time_threshold
        }
        #[cfg(not(feature = "local"))]
        {
            elapsed_time >= self.time_threshold
        }
    }
    #[inline]
    fn get_time(&self) -> f64 {
        let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            elapsed_time * 0.55
        }
        #[cfg(not(feature = "local"))]
        {
            elapsed_time
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    pub fn in_map(&self, height: usize, width: usize) -> bool {
        self.row < height && self.col < width
    }
    pub fn to_index(&self, width: usize) -> CoordIndex {
        CoordIndex(self.row * width + self.col)
    }
}

impl std::ops::Add<CoordDiff> for Coord {
    type Output = Coord;
    fn add(self, rhs: CoordDiff) -> Self::Output {
        Coord::new(
            self.row.wrapping_add_signed(rhs.dr),
            self.col.wrapping_add_signed(rhs.dc),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordDiff {
    dr: isize,
    dc: isize,
}

impl CoordDiff {
    pub const fn new(dr: isize, dc: isize) -> Self {
        Self { dr, dc }
    }
}

pub const ADJ: [CoordDiff; 4] = [
    CoordDiff::new(1, 0),
    CoordDiff::new(!0, 0),
    CoordDiff::new(0, 1),
    CoordDiff::new(0, !0),
];

pub struct CoordIndex(pub usize);

impl CoordIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn to_coord(&self, width: usize) -> Coord {
        Coord {
            row: self.0 / width,
            col: self.0 % width,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DynamicMap2d<T> {
    pub size: usize,
    map: Vec<T>,
}

impl<T> DynamicMap2d<T> {
    pub fn new(map: Vec<T>, size: usize) -> Self {
        assert_eq!(size * size, map.len());
        Self { size, map }
    }
}

impl<T: Clone> DynamicMap2d<T> {
    pub fn new_with(v: T, size: usize) -> Self {
        let map = vec![v; size * size];
        Self::new(map, size)
    }
}

impl<T> std::ops::Index<Coord> for DynamicMap2d<T> {
    type Output = T;

    #[inline]
    fn index(&self, coordinate: Coord) -> &Self::Output {
        &self[coordinate.to_index(self.size)]
    }
}

impl<T> std::ops::IndexMut<Coord> for DynamicMap2d<T> {
    #[inline]
    fn index_mut(&mut self, coordinate: Coord) -> &mut Self::Output {
        let size = self.size;
        &mut self[coordinate.to_index(size)]
    }
}

impl<T> std::ops::Index<CoordIndex> for DynamicMap2d<T> {
    type Output = T;

    fn index(&self, index: CoordIndex) -> &Self::Output {
        unsafe { self.map.get_unchecked(index.0) }
    }
}

impl<T> std::ops::IndexMut<CoordIndex> for DynamicMap2d<T> {
    #[inline]
    fn index_mut(&mut self, index: CoordIndex) -> &mut Self::Output {
        unsafe { self.map.get_unchecked_mut(index.0) }
    }
}
