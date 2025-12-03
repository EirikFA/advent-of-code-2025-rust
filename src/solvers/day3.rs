use std::{
  fs::File,
  io::{self, BufRead},
};

use super::Solver;

pub struct Day3;

impl Solver for Day3 {
  type ParsedInput = Vec<Vec<u32>>;

  type Output1 = u32;

  type Output2 = u64;

  fn parse(input: &mut File) -> Self::ParsedInput {
    io::BufReader::new(input)
      .lines()
      .flatten()
      .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
      .collect()
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(357)
  }

  fn part_1(banks: &Self::ParsedInput) -> Self::Output1 {
    let mut sum = 0;

    for bank in banks {
      let mut max = 0;
      let mut max_i = 0;
      for (i, &n) in bank.iter().enumerate() {
        if n > max {
          max = n;
          max_i = i;
        }
      }

      let mut max2 = 0;
      let swap = max_i == bank.len() - 1;
      let range = if swap {
        0..max_i
      } else {
        max_i + 1..bank.len()
      };
      for &n in &bank[range] {
        if n > max2 {
          max2 = n;
        }
      }

      let (max1_new, max2_new) = if swap { (max2, max) } else { (max, max2) };

      sum += max1_new * 10 + max2_new;
    }

    sum
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(3121910778619)
  }

  fn part_2(banks: &Self::ParsedInput) -> Self::Output2 {
    let mut sum = 0;

    for bank in banks {
      let mut digits = vec![0u32; 12];
      let mut next_start = 0;

      for i in 0..digits.len() {
        let mut max = 0;
        let mut max_i = 0;

        for (j, &n) in bank[next_start..bank.len() - 11 + i].iter().enumerate() {
          if n > max {
            max = n;
            max_i = j;
          }
        }
        digits[i] = max;
        next_start += max_i + 1;
      }

      let mut exp = 12;
      for &n in &digits {
        exp -= 1;
        sum += (n as u64) * 10u64.pow(exp);
      }
    }

    sum
  }
}

impl Day3 {}
