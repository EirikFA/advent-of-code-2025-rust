use std::{fs::File, io::Read, str::FromStr};

use super::Solver;

pub struct Day6;

impl Solver for Day6 {
  type ParsedInput = Vec<(Operator, Vec<String>)>;

  type Output1 = u64;

  type Output2 = u64;

  fn parse(input: &mut File) -> Self::ParsedInput {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();
    let rows = str.lines().count();
    let cols = str
      .lines()
      .next()
      .unwrap()
      .trim()
      .split_whitespace()
      .count();

    let mut number_cols: Vec<Vec<String>> = vec![vec![String::new(); rows - 1]; cols];
    let mut operators: Vec<Operator> = Vec::new();

    for (row, line) in str.lines().enumerate() {
      if row == rows - 1 {
        operators = line
          .trim()
          .split_whitespace()
          .map(|s| s.parse().unwrap())
          .collect();
      } else {
        let mut split_indices = Vec::new();
        for (col, char) in line.chars().enumerate() {
          if char.is_whitespace()
            && str
              .lines()
              .all(|l| l.chars().nth(col).unwrap().is_whitespace())
          {
            split_indices.push(col);
          }
        }
        split_indices.push(line.len());

        let mut last_split = 0;
        for (col, &split_index) in split_indices.iter().enumerate() {
          let num_str = line[last_split..split_index].to_string();
          number_cols[col][row] = num_str;
          last_split = split_index + 1;
        }
      }
    }

    operators.into_iter().zip(number_cols).collect()
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(4277556)
  }

  fn part_1(cols: &Self::ParsedInput) -> Self::Output1 {
    cols.iter().fold(0, |acc, (op, nums)| {
      let init = match op {
        Operator::Add => 0,
        Operator::Multiply => 1,
      };
      acc
        + nums.iter().fold(init, |acc_col, num| {
          op.apply(acc_col, num.trim().parse().unwrap())
        })
    })
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(3263827)
  }

  fn part_2(cols: &Self::ParsedInput) -> Self::Output2 {
    let mut new_num_cols: Vec<Vec<String>> = vec![vec![]; cols.len()];
    for (col, (_, nums)) in cols.iter().enumerate() {
      let len = nums.iter().next().unwrap().len();
      for i in 0..len {
        let number: String = nums.iter().map(|n| n.chars().nth(i).unwrap()).collect();
        new_num_cols[col].push(number);
      }
    }

    Self::part_1(
      &cols
        .iter()
        .zip(new_num_cols.into_iter())
        .map(|((op, _), nums)| (op.clone(), nums))
        .collect(),
    )
  }
}

#[derive(Debug, Clone)]
pub enum Operator {
  Add,
  Multiply,
}

impl FromStr for Operator {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "+" => Ok(Operator::Add),
      "*" => Ok(Operator::Multiply),
      _ => Err(()),
    }
  }
}

impl Operator {
  fn apply(&self, a: u64, b: u64) -> u64 {
    match self {
      Operator::Add => a + b,
      Operator::Multiply => a * b,
    }
  }
}
