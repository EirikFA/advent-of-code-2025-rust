use std::{
  collections::HashMap,
  fs::File,
  io::{self, BufRead},
};

use super::Solver;

pub struct Day8;

impl Solver for Day8 {
  type ParsedInput = Vec<Point>;

  type Output1 = u64;

  type Output2 = u32;

  fn parse(input: &mut File) -> Self::ParsedInput {
    io::BufReader::new(input)
      .lines()
      .flatten()
      .map(|line| {
        let coords = line
          .split(',')
          .map(|num| num.parse::<u32>().unwrap())
          .collect::<Vec<u32>>();
        Point {
          x: coords[0],
          y: coords[1],
          z: coords[2],
        }
      })
      .collect()
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(40)
  }

  fn part_1(boxes: &Self::ParsedInput) -> Self::Output1 {
    Self::part_1_flag(boxes, false)
  }

  fn part_1_flag(boxes: &Self::ParsedInput, is_test: bool) -> Self::Output1 {
    let mut circuits = DisjointSetForest::new(boxes.len());
    let count = if is_test { 10 } else { 1000 };
    for &((ai, _), (bi, _)) in Self::make_pairs(boxes).iter().take(count) {
      circuits.union(ai, bi);
    }

    let mut circuits = Vec::from(circuits);
    circuits.sort_by_key(|c| c.len());
    circuits.reverse();
    circuits
      .iter()
      .take(3)
      .fold(1, |acc, c| acc * (c.len() as u64))
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(25272)
  }

  fn part_2(boxes: &Self::ParsedInput) -> Self::Output2 {
    let mut circuits = DisjointSetForest::new(boxes.len());
    for &((ai, a), (bi, b)) in Self::make_pairs(boxes).iter() {
      circuits.union(ai, bi);
      if circuits.sizes.iter().any(|n| *n == boxes.len()) {
        return a.x * b.x;
      }
    }
    panic!("Not a single circuit");
  }
}

impl Day8 {
  fn make_pairs(boxes: &Vec<Point>) -> Vec<((usize, &Point), (usize, &Point))> {
    let mut pairs = boxes
      .iter()
      .enumerate()
      .flat_map(|(i, box_a)| {
        boxes
          .iter()
          .enumerate()
          .skip(i + 1)
          .map(move |(j, box_b)| ((i, box_a), (j, box_b)))
      })
      .collect::<Vec<_>>();
    pairs.sort_by(|&((_, a1), (_, b1)), &((_, a2), (_, b2))| {
      a1.distance(b1).partial_cmp(&a2.distance(b2)).unwrap()
    });
    pairs
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
  x: u32,
  y: u32,
  z: u32,
}

impl Point {
  fn distance(&self, other: &Point) -> f64 {
    (((self.x as i64 - other.x as i64).pow(2)
      + (self.y as i64 - other.y as i64).pow(2)
      + (self.z as i64 - other.z as i64).pow(2)) as f64)
      .sqrt()
  }
}

struct DisjointSetForest {
  parents: Vec<usize>,
  sizes: Vec<usize>,
}

impl DisjointSetForest {
  fn new(n: usize) -> Self {
    Self {
      parents: (0..n).collect(),
      sizes: vec![1; n],
    }
  }

  fn find(&mut self, x: usize) -> usize {
    if self.parents[x] != x {
      return self.find(self.parents[x]);
    }
    self.parents[x]
  }

  fn union(&mut self, a: usize, b: usize) {
    let root_a = self.find(a);
    let root_b = self.find(b);
    if root_a != root_b {
      if self.sizes[root_a] >= self.sizes[root_b] {
        self.parents[root_b] = root_a;
        self.sizes[root_a] += self.sizes[root_b];
      } else {
        self.parents[root_a] = root_b;
        self.sizes[root_b] += self.sizes[root_a];
      }
    }
  }
}

impl From<DisjointSetForest> for Vec<Vec<usize>> {
  fn from(mut dsu: DisjointSetForest) -> Self {
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..dsu.parents.len() {
      let root = dsu.find(i);
      groups.entry(root).or_default().push(i);
    }
    groups.into_values().collect()
  }
}
