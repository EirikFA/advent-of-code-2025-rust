use std::{fmt::Display, fs::File, time::Instant};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub trait Solver {
  type ParsedInput;
  type Output1: Display;
  type Output2: Display;

  fn parse(input: &mut File) -> Self::ParsedInput;

  fn part_1_test_output() -> Option<Self::Output1>;
  fn part_1(parsed_input: &Self::ParsedInput) -> Self::Output1;
  fn part_1_flag(parsed_input: &Self::ParsedInput, _is_test: bool) -> Self::Output1 {
    Self::part_1(parsed_input)
  }

  fn part_2_test_output() -> Option<Self::Output2>;
  fn part_2(parsed_input: &Self::ParsedInput) -> Self::Output2;
  fn part_2_flag(parsed_input: &Self::ParsedInput, _is_test: bool) -> Self::Output2 {
    Self::part_2(parsed_input)
  }

  fn parse_file(path: &str) -> Self::ParsedInput {
    match File::open(path) {
      Ok(mut file) => Self::parse(&mut file),
      Err(error) => panic!("Error opening input file: {}", error),
    }
  }

  fn test(day_path: &str) {
    let part_1_expected = Self::part_1_test_output();
    if let Some(part_1_expected) = part_1_expected {
      let part_1_input = Self::parse_file(&format!("{}/part_1_test.txt", day_path));
      assert_eq!(
        Self::part_1_flag(&part_1_input, true).to_string(),
        part_1_expected.to_string()
      );
    }

    let part_2_expected = Self::part_2_test_output();
    if let Some(part_2_expected) = part_2_expected {
      let part_2_input = Self::parse_file(&format!("{}/part_2_test.txt", day_path));
      assert_eq!(
        Self::part_2_flag(&part_2_input, true).to_string(),
        part_2_expected.to_string()
      );
    }
  }

  fn run(day_path: &str) {
    Self::test(day_path);

    let path = format!("{}/input.txt", day_path);
    let input = Self::parse_file(&path);

    let before_1 = Instant::now();
    println!("Part 1: {}", Self::part_1_flag(&input, false));
    println!("Part 1 took {:?}", before_1.elapsed());

    let before_2 = Instant::now();
    println!("Part 2: {}", Self::part_2_flag(&input, false));
    println!("Part 2 took {:?}", before_2.elapsed());
  }
}
