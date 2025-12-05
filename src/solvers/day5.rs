use std::{
  fs::File,
  io::{self, BufRead},
  ops::RangeInclusive,
};

use super::Solver;

pub struct Day5;

impl Solver for Day5 {
  type ParsedInput = (Vec<RangeInclusive<u64>>, Vec<u64>);

  type Output1 = u64;

  type Output2 = u64;

  fn parse(input: &mut File) -> Self::ParsedInput {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut reading_ranges = true;
    for line in io::BufReader::new(input).lines().flatten() {
      if line.is_empty() {
        reading_ranges = false;
        continue;
      }

      if reading_ranges {
        let parts: Vec<&str> = line.split('-').collect();
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();
        ranges.push(start..=end);
      } else {
        let id: u64 = line.parse().unwrap();
        ids.push(id);
      }
    }
    (ranges, ids)
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(3)
  }

  fn part_1((ranges, ids): &Self::ParsedInput) -> Self::Output1 {
    ids.iter().fold(0, |acc, id| {
      if ranges.iter().any(|r| r.contains(id)) {
        acc + 1
      } else {
        acc
      }
    })
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(14)
  }

  fn part_2((ranges, _): &Self::ParsedInput) -> Self::Output2 {
    let mut ranges = ranges.iter().cloned().collect::<Vec<_>>();
    ranges.sort_by_key(|r| *r.start());

    let mut new_ranges = vec![ranges[0].clone()];
    for r in ranges {
      let last = new_ranges.last_mut().unwrap();
      if r.start() <= last.end() {
        *last = *last.start()..=*last.end().max(r.end());
        continue;
      }
      new_ranges.push(r.clone());
    }

    new_ranges
      .iter()
      .fold(0, |acc, r| acc + (r.end() - r.start() + 1))
  }
}
