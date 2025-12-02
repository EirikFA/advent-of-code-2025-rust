use std::{fs::File, io::Read};

use super::Solver;

pub struct Day2;

impl Solver for Day2 {
  type ParsedInput = Vec<(u64, u64)>;

  type Output1 = u64;

  type Output2 = u64;

  fn parse(input: &mut File) -> Self::ParsedInput {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();
    str
      .split(',')
      .map(|pair| {
        let mut nums = pair.split('-');
        let first = nums.next().unwrap().parse::<u64>().unwrap();
        let second = nums.next().unwrap().parse::<u64>().unwrap();
        (first, second)
      })
      .collect()
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(1227775554)
  }

  fn part_1(parsed_input: &Self::ParsedInput) -> Self::Output1 {
    let mut sum: u64 = 0;
    for (start, end) in parsed_input {
      for num in *start..=*end {
        if Self::is_repeated_once(num) {
          sum += num;
        }
      }
    }
    sum
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(4174379265)
  }

  fn part_2(parsed_input: &Self::ParsedInput) -> Self::Output2 {
    let mut sum: u64 = 0;
    for (start, end) in parsed_input {
      for num in *start..=*end {
        if Self::is_repeated(num) {
          sum += num;
        }
      }
    }
    sum
  }
}

impl Day2 {
  // fn is_repeated_once(num: u64) -> bool {
  //   let str = num.to_string();
  //   if str.len() % 2 != 0 {
  //     return false;
  //   }
  //   let parts = str.split_at(str.len() / 2);
  //   parts.0 == parts.1
  // }

  // Much better :)
  fn is_repeated_once(num: u64) -> bool {
    let len = num.ilog10() + 1;
    if len % 2 != 0 {
      return false;
    }

    let split_scale = 10u64.pow(len / 2);
    num / split_scale == num % split_scale
  }

  fn is_repeated(num: u64) -> bool {
    let str = num.to_string();
    let bytes = str.as_bytes();
    for i in 1..=bytes.len() / 2 {
      let mut chunks = bytes.chunks(i);
      if chunks.all(|chunk| chunk == &bytes[0..i]) {
        return true;
      }
    }
    false
  }
}
