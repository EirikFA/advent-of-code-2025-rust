use std::{
  collections::{BinaryHeap, HashSet, VecDeque},
  fs::File,
  io::{self, BufRead},
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use rustc_hash::FxHashMap;

use super::Solver;

type Lights = u16;
type Buttons = Vec<Vec<u8>>;
type Joltages = u128;
type Machine = (Lights, Buttons, Joltages);

pub struct Day10;

impl Solver for Day10 {
  type ParsedInput = Vec<Machine>;

  type Output1 = u16;

  type Output2 = u32;

  fn parse(input: &mut File) -> Self::ParsedInput {
    let button_regex = Regex::new(r"\(([\d,]+)\)").unwrap();
    let mut machines = Vec::new();

    for line in io::BufReader::new(input).lines().flatten() {
      let (lights_str, buttons_str) = line.split_once(" ").unwrap();
      let lights: Lights =
        lights_str[1..lights_str.len() - 1]
          .char_indices()
          .fold(
            0,
            |mask, (i, c)| if c == '#' { mask | (1 << i) } else { mask },
          );
      let buttons: Buttons = button_regex
        .captures_iter(buttons_str)
        .map(|c| c[1].split(',').map(|n| n.parse::<u8>().unwrap()).collect())
        .collect();

      let joltages_str = buttons_str.split(" {").nth(1).unwrap();
      let joltages: Joltages = joltages_str[..joltages_str.len() - 1]
        .split(',')
        .enumerate()
        .fold(0u128, |acc, (i, n)| {
          acc | ((n.parse::<u16>().unwrap() as u128) << (i * 9))
        });
      machines.push((lights, buttons, joltages));
    }

    machines
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(7)
  }

  fn part_1(machines: &Self::ParsedInput) -> Self::Output1 {
    // Parallel for fun (~500us instead of ~1.5ms in release)
    machines.par_iter().map(Self::bfs_lights).sum()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(33)
  }

  fn part_2(machines: &Self::ParsedInput) -> Self::Output2 {
    let mut machines = machines.clone();
    machines.sort_by_key(|m| m.1.len());
    machines
      .iter()
      .enumerate()
      .map(|(i, machine)| {
        let max_button_size = machine.1.iter().map(|b| b.len()).max().unwrap();
        let cost = Self::astar_joltages(machine, max_button_size) as u32;
        println!("Machine {}", i);
        cost
      })
      .sum()
  }
}

impl Day10 {
  fn bfs_lights((goal_lights, buttons, _): &Machine) -> u16 {
    let mut frontier = VecDeque::from([(0u16, 0u16)]);
    let mut reached = HashSet::from([0u16]);
    while let Some((lights, depth)) = frontier.pop_front() {
      if lights == *goal_lights {
        return depth;
      }
      for button in buttons {
        let mut next_lights = lights;
        for &toggle in button {
          next_lights ^= 1 << toggle;
        }
        if !reached.contains(&next_lights) {
          reached.insert(next_lights);
          frontier.push_back((next_lights, depth + 1));
        }
      }
    }

    panic!("No path found");
  }

  fn astar_joltages((_, buttons, goal_joltages): &Machine, max_button_size: usize) -> u16 {
    let init_node = AStarNode {
      joltages: 0,
      cost: 0,
      heuristic: Self::astar_heuristic(0, goal_joltages, max_button_size),
    };
    let mut frontier = BinaryHeap::from([init_node]);
    let mut reached = FxHashMap::default();
    reached.insert(0u128, 0u16);

    while let Some(n) = frontier.pop() {
      if reached.contains_key(&n.joltages) && n.cost > reached[&n.joltages] {
        continue;
      }
      if n.joltages == *goal_joltages {
        return n.cost;
      }

      'buttons: for button in buttons {
        let mut child_joltages = n.joltages;
        for &i in button {
          if n.joltages.joltage(i as usize) == goal_joltages.joltage(i as usize) {
            continue 'buttons;
          }
          child_joltages.increment_joltage(i as usize);
        }

        let child_cost = n.cost + 1;
        if !reached.contains_key(&child_joltages) || child_cost < reached[&child_joltages] {
          let child = AStarNode {
            joltages: child_joltages,
            cost: child_cost,
            heuristic: Self::astar_heuristic(child_joltages, goal_joltages, max_button_size),
          };
          reached.insert(child_joltages, child_cost);
          frontier.push(child);
        }
      }
    }

    panic!("No path found");
  }

  fn astar_heuristic(joltages: Joltages, goal_joltages: &Joltages, max_button_size: usize) -> u16 {
    let mut remaining_max = 0;
    let mut remaining_sum = 0;
    for i in 0..10 {
      let diff = goal_joltages.joltage(i) - joltages.joltage(i);
      remaining_sum += diff;
      if diff > remaining_max {
        remaining_max = diff;
      }
    }

    let min_presses = (remaining_sum as f32 / max_button_size as f32).ceil() as u16;
    remaining_max.max(min_presses)
  }
}

trait IntJoltages {
  fn joltage(&self, i: usize) -> u16;
  fn increment_joltage(&mut self, i: usize) -> ();
}

impl IntJoltages for u128 {
  fn joltage(&self, i: usize) -> u16 {
    ((self >> (i * 9)) & 0x1FF) as u16
  }

  fn increment_joltage(&mut self, i: usize) -> () {
    *self += 1u128 << (i * 9);
  }
}

#[derive(Eq, PartialEq)]
struct AStarNode {
  joltages: u128,
  cost: u16,
  heuristic: u16,
}

impl Ord for AStarNode {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    (other.cost + other.heuristic)
      .cmp(&(self.cost + self.heuristic))
      .then_with(|| self.joltages.cmp(&other.joltages))
  }
}

impl PartialOrd for AStarNode {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
