#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]

use proconio::{input, marker::Usize1};
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

const N: usize = 20;

fn solve() {
    let mut rng = rand_pcg::Pcg64Mcg::new(0);
    let input = read_input(&mut rng);
    let mut init_hash = 0;
    for i in 0..N {
        init_hash ^= input.hashes_plus[i][0];
    }

    let init_node = Node {
        track_id: !0,
        score: N as i64,
        hash: init_hash,
        state: [0; N],
    };
    let mut beam = BeamSearch::new(init_node);
    let ret = beam.solve(&input);
    for &op in &ret {
        if op == 0 {
            println!("A");
        } else {
            println!("B");
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    track_id: usize,
    score: i64,
    hash: u64,
    state: [i8; N],
}
impl Node {
    fn new_node(&self, cand: &Cand, mut state: [i8; N], input: &Input, turn: usize) -> Node {
        let add = if cand.op == 0 { 1 } else { -1 };
        for &idx in &input.PQR[turn] {
            state[idx] += add;
        }
        Node {
            track_id: !0,
            score: cand.eval_score,
            hash: cand.hash,
            state,
        }
    }
}

#[derive(Debug, Clone)]
struct Cand {
    op: u8,
    parent: usize,
    eval_score: i64,
    hash: u64,
}
impl Cand {
    fn raw_score(&self, _input: &Input) -> i64 {
        self.eval_score
    }
}

#[derive(Debug)]
struct BeamSearch {
    track: Vec<(usize, u8)>,
    nodes: Vec<Node>,
    next_nodes: Vec<Node>,
}
impl BeamSearch {
    fn new(node: Node) -> BeamSearch {
        BeamSearch {
            nodes: vec![node],
            track: vec![],
            next_nodes: vec![],
        }
    }

    fn enum_cands(&self, input: &Input, cands: &mut Vec<Cand>, turn: usize) {
        for i in 0..self.nodes.len() {
            self.append_cands(input, i, cands, turn);
        }
    }

    fn append_cands(&self, input: &Input, idx: usize, cands: &mut Vec<Cand>, turn: usize) {
        let parent_node = &self.nodes[idx];
        let parent_score = parent_node.state.iter().filter(|&&x| x == 0).count() as i64;
        let parent_partial_score = {
            let mut ret = 0;
            for &idx in &input.PQR[turn] {
                if parent_node.state[idx] == 0 {
                    ret += 1;
                }
            }
            ret
        };

        // +1
        let partial_score = {
            let mut ret = 0;
            for &idx in &input.PQR[turn] {
                if parent_node.state[idx] + 1 == 0 {
                    ret += 1;
                }
            }
            ret
        };
        let next_score = parent_score + (partial_score - parent_partial_score);
        let hash = {
            let mut ret = parent_node.hash;
            for &idx in &input.PQR[turn] {
                ret ^= if parent_node.state[idx] >= 0 {
                    input.hashes_plus[idx][parent_node.state[idx] as usize]
                } else {
                    input.hashes_minus[idx][parent_node.state[idx].unsigned_abs() as usize]
                };
                ret ^= if parent_node.state[idx] + 1 >= 0 {
                    input.hashes_plus[idx][(parent_node.state[idx] + 1) as usize]
                } else {
                    input.hashes_minus[idx][(parent_node.state[idx] + 1).unsigned_abs() as usize]
                };
            }
            ret
        };

        let cand = Cand {
            op: 0,
            parent: idx,
            eval_score: parent_node.score + next_score,
            hash,
        };
        cands.push(cand);

        // -1
        let partial_score = {
            let mut ret = 0;
            for &idx in &input.PQR[turn] {
                if parent_node.state[idx] - 1 == 0 {
                    ret += 1;
                }
            }
            ret
        };
        let next_score = parent_score + (partial_score - parent_partial_score);
        let hash = {
            let mut ret = parent_node.hash;
            for &idx in &input.PQR[turn] {
                ret ^= if parent_node.state[idx] >= 0 {
                    input.hashes_plus[idx][parent_node.state[idx] as usize]
                } else {
                    input.hashes_minus[idx][parent_node.state[idx].unsigned_abs() as usize]
                };
                ret ^= if parent_node.state[idx] >= 1 {
                    input.hashes_plus[idx][(parent_node.state[idx] - 1) as usize]
                } else {
                    input.hashes_minus[idx][(parent_node.state[idx] - 1).unsigned_abs() as usize]
                };
            }
            ret
        };

        let cand = Cand {
            op: 1,
            parent: idx,
            eval_score: parent_node.score + next_score,
            hash,
        };
        cands.push(cand);
    }

    fn update<I: Iterator<Item = Cand>>(&mut self, cands: I, input: &Input, turn: usize) {
        self.next_nodes.clear();
        for cand in cands {
            let parent_node = &self.nodes[cand.parent];
            let mut new_node = parent_node.new_node(&cand, parent_node.state, input, turn);
            self.track.push((parent_node.track_id, cand.op));
            new_node.track_id = self.track.len() - 1;
            self.next_nodes.push(new_node);
        }
        std::mem::swap(&mut self.nodes, &mut self.next_nodes);
    }

    fn restore(&self, mut idx: usize) -> Vec<u8> {
        idx = self.nodes[idx].track_id;
        let mut ret = vec![];
        while idx != !0 {
            ret.push(self.track[idx].1);
            idx = self.track[idx].0;
        }
        ret.reverse();
        ret
    }

    fn solve(&mut self, input: &Input) -> Vec<u8> {
        use std::cmp::Reverse;
        let M = 100000;

        let mut cands = Vec::<Cand>::new();
        let mut set = rustc_hash::FxHashSet::default();
        for t in 0..input.T {
            if t != 0 {
                cands.sort_unstable_by_key(|a| Reverse(a.eval_score));
                set.clear();
                self.update(
                    cands
                        .iter()
                        .filter(|cand| set.insert(cand.hash))
                        .take(M)
                        .cloned(),
                    input,
                    t - 1,
                );
            }
            cands.clear();
            self.enum_cands(input, &mut cands, t);
        }

        let best = cands.iter().max_by_key(|a| a.raw_score(input)).unwrap();
        eprintln!("score = {}", best.raw_score(input));

        let mut ret = self.restore(best.parent);
        ret.push(best.op);

        ret
    }
}

struct Input {
    T: usize,
    PQR: Vec<Vec<usize>>,
    hashes_plus: Vec<Vec<u64>>,
    hashes_minus: Vec<Vec<u64>>,
}

fn read_input(rng: &mut rand_pcg::Pcg64Mcg) -> Input {
    input! {
        T: usize,
        PQR: [[Usize1; 3]; T]
    }
    let mut hashes_plus = vec![vec![!0; T + 1]; N];
    let mut hashes_minus = vec![vec![!0; T + 1]; N];
    for i in 0..N {
        for j in 0..=T {
            hashes_plus[i][j] = rng.gen::<u64>();
            hashes_minus[i][j] = rng.gen::<u64>();
        }
    }

    Input {
        T,
        PQR,
        hashes_plus,
        hashes_minus,
    }
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
