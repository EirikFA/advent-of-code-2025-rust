use std::{
  fs::File,
  io::{self, BufRead},
};

use super::Solver;

#[derive(PartialEq)]
pub enum Direction {
  Left,
  Right,
}

pub struct Day1;

impl Solver for Day1 {
  type ParsedInput = Vec<(Direction, u32)>;

  type Output1 = u32;

  type Output2 = u32;

  fn parse(input: &mut File) -> Self::ParsedInput {
    io::BufReader::new(input)
      .lines()
      .flatten()
      .map(|line| {
        let mut chars = line.chars();
        let direction = match chars.next().expect("Direction") {
          'L' => Direction::Left,
          'R' => Direction::Right,
          _ => panic!("Unexpected direction"),
        };
        let value = chars.as_str().parse::<u32>().expect("Number");
        (direction, value)
      })
      .collect()
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(3)
  }

  fn part_1(parsed_input: &Self::ParsedInput) -> Self::Output1 {
    parsed_input
      .iter()
      .fold((50, 0), |(dial, count0), (dir, val)| {
        let mut dial = dial;
        dial = match dir {
          Direction::Left => (dial + 100 - val % 100) % 100,
          Direction::Right => (dial + val) % 100,
        };
        if dial == 0 {
          (dial, count0 + 1)
        } else {
          (dial, count0)
        }
      })
      .1
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(6)
  }

  fn part_2(parsed_input: &Self::ParsedInput) -> Self::Output2 {
    let mut count: u32 = 0;
    let mut dial: u32 = 50;
    for (dir, val) in parsed_input {
      for _ in 0..*val {
        dial = match dir {
          Direction::Left => (dial + 100 - 1) % 100,
          Direction::Right => (dial + 1) % 100,
        };
        if dial == 0 {
          count += 1;
        }
      }
    }
    count
  }
}
