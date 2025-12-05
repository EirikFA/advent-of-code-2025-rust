use std::{
  collections::{HashSet, VecDeque},
  fs::File,
  io::{self, BufRead},
};

use super::Solver;

pub struct Day4;

impl Solver for Day4 {
  type ParsedInput = HashSet<(u8, u8)>;

  type Output1 = u32;

  type Output2 = u32;

  fn parse(input: &mut File) -> Self::ParsedInput {
    let mut paper_rolls = HashSet::new();
    for (row, line) in io::BufReader::new(input).lines().flatten().enumerate() {
      for (col, char) in line.chars().enumerate() {
        if char == '@' {
          paper_rolls.insert((col as u8 + 1, row as u8 + 1));
        }
      }
    }
    paper_rolls
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(13)
  }

  fn part_1(rolls: &Self::ParsedInput) -> Self::Output1 {
    rolls
      .iter()
      .filter(|(x, y)| Self::neighbor_rolls(rolls, *x, *y).len() < 4)
      .count() as u32
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(43)
  }

  fn part_2(rolls: &Self::ParsedInput) -> Self::Output2 {
    let mut count = 0;
    let mut q: VecDeque<_> = rolls.iter().copied().collect();
    let mut rolls = rolls.clone();
    while !q.is_empty() {
      let (x, y) = q.pop_front().unwrap();
      let neighbors = Self::neighbor_rolls(&rolls, x, y);
      if neighbors.len() < 4 && rolls.remove(&(x, y)) {
        count += 1;
        q.extend(neighbors);
      }
    }
    count
  }
}

impl Day4 {
  const NEIGHBOR_OFFSETS: [(i16, i16); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
  ];

  fn neighbor_rolls(rolls: &HashSet<(u8, u8)>, x: u8, y: u8) -> Vec<(u8, u8)> {
    Self::NEIGHBOR_OFFSETS
      .iter()
      .filter_map(|(dx, dy)| {
        let nx = x as i16 + *dx;
        let ny = y as i16 + *dy;
        let neighbor = (nx as u8, ny as u8);
        if rolls.contains(&neighbor) {
          return Some(neighbor);
        }
        None
      })
      .collect()
  }
}
