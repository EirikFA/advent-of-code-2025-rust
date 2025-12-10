use std::{
  fs::File,
  io::{self, BufRead},
};

use super::Solver;

pub struct Day9;

impl Solver for Day9 {
  type ParsedInput = Vec<(u32, u32)>;

  type Output1 = u64;

  type Output2 = u32;

  fn parse(input: &mut File) -> Self::ParsedInput {
    io::BufReader::new(input)
      .lines()
      .flatten()
      .map(|line| {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<u32>().unwrap();
        let y = parts.next().unwrap().parse::<u32>().unwrap();
        (x, y)
      })
      .collect()
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(50)
  }

  fn part_1(red_tiles: &Self::ParsedInput) -> Self::Output1 {
    let mut pairs = red_tiles
      .iter()
      .enumerate()
      .flat_map(|(i, a)| red_tiles.iter().skip(i + 1).map(move |b| (a, b)))
      .collect::<Vec<_>>();
    pairs.sort_by_key(|&(a, b)| Self::area(*a, *b));
    let &(a, b) = pairs.last().unwrap();
    Self::area(*a, *b)
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(0)
  }

  fn part_2(_red_tiles: &Self::ParsedInput) -> Self::Output2 {
    0
  }
}

impl Day9 {
  fn area((ax, ay): (u32, u32), (bx, by): (u32, u32)) -> u64 {
    (ax as i64 - bx as i64 + 1).abs() as u64 * (ay as i64 - by as i64 + 1).abs() as u64
  }
}
