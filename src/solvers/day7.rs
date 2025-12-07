use std::{
  collections::{HashMap, HashSet},
  fs::File,
  io::BufRead,
};

use super::Solver;

pub struct Day7;

impl Solver for Day7 {
  type ParsedInput = (u16, Vec<HashSet<u16>>, u16);

  type Output1 = u32;

  type Output2 = u64;

  fn parse(input: &mut File) -> Self::ParsedInput {
    let mut start = 0;
    let mut splitter_rows = Vec::new();
    let mut rows = 0;
    for (y, line) in std::io::BufReader::new(input).lines().flatten().enumerate() {
      splitter_rows.push(HashSet::new());
      for (x, c) in line.chars().enumerate() {
        if c == 'S' {
          start = x as u16;
        } else if c == '^' {
          splitter_rows[y].insert(x as u16);
        }
      }
      rows += 1;
    }
    (start, splitter_rows, rows)
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(21)
  }

  fn part_1((start, splitter_rows, rows): &Self::ParsedInput) -> Self::Output1 {
    let mut split_count = 0;
    let mut beams: HashSet<u16> = HashSet::new();
    beams.insert(*start);

    for y in 1..*rows {
      let splitters = &splitter_rows[y as usize];
      if splitters.is_empty() {
        continue;
      }

      let mut next_beams: HashSet<u16> = HashSet::new();
      for beam in beams {
        if splitters.contains(&beam) {
          split_count += 1;
          next_beams.insert(beam - 1);
          next_beams.insert(beam + 1);
        } else {
          next_beams.insert(beam);
        }
      }
      beams = next_beams;
    }

    split_count
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(40)
  }

  fn part_2((start, splitter_rows, rows): &Self::ParsedInput) -> Self::Output2 {
    let mut beam_timelines: HashMap<u16, u64> = HashMap::new();
    beam_timelines.insert(*start, 1);

    for y in 1..*rows {
      let splitters = &splitter_rows[y as usize];
      if splitters.is_empty() {
        continue;
      }

      let mut next_beam_timelines: HashMap<u16, u64> = HashMap::new();
      for (beam, count) in beam_timelines {
        if splitters.contains(&beam) {
          next_beam_timelines
            .entry(beam - 1)
            .and_modify(|c| *c += count)
            .or_insert(count);
          next_beam_timelines
            .entry(beam + 1)
            .and_modify(|c| *c += count)
            .or_insert(count);
        } else {
          next_beam_timelines
            .entry(beam)
            .and_modify(|c| *c += count)
            .or_insert(count);
        }
      }
      beam_timelines = next_beam_timelines;
    }

    beam_timelines.values().sum()
  }
}
