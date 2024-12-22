use advent_of_code_2024::inp;
use core::num;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

fn main() {
  let vec = inp::parse_file("inputs/day19.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn is_possible_design(
  pattern_map: &mut HashSet<String>,
  fails: &mut HashSet<String>,
  desired_pattern: &String,
) -> bool {
  if fails.contains(desired_pattern) {
    return false;
  }
  match pattern_map.get(desired_pattern) {
    Some(_) => {
      return true;
    }
    None => {
      if desired_pattern.len() == 1 {
        return false;
      }
      for partition_start in 1..desired_pattern.len() {
        let first_half = &desired_pattern[..partition_start];
        let second_half = &desired_pattern[partition_start..];
        let mut found1 = false;
        if is_possible_design(pattern_map, fails, &first_half.to_string()) {
          pattern_map.insert(first_half.to_string());
          found1 = true;
        } else {
          fails.insert(first_half.to_string());
        }
        let mut found2 = false;
        if is_possible_design(pattern_map, fails, &second_half.to_string()) {
          pattern_map.insert(second_half.to_string());
          found2 = true;
        } else {
          fails.insert(second_half.to_string());
        }
        if found1 && found2 {
          return true;
        }
      }
      return false;
    }
  }
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let mut patterns = input[0].split(", ").collect::<Vec<&str>>();
  let mut pattern_set = HashSet::new();
  let mut fails = HashSet::new();
  patterns.sort_by(|&x, &y| x.len().cmp(&y.len()));
  for &p in &patterns {
    pattern_set.insert(p.to_string());
  }

  input[2..]
    .iter()
    .filter(|&pattern| {
      is_possible_design(&mut pattern_set, &mut fails, pattern)
    })
    .count() as i32
}

fn get_num_possibilities(
  possibility_map: &mut HashMap<String, i32>,
  fails: &HashSet<String>,
  desired_pattern: &String,
) -> i32 {
  if fails.contains(desired_pattern) {
    return 0;
  }
  match possibility_map.get(desired_pattern) {
    Some(&possibilities) => {
      return possibilities;
    }
    None => {
      if desired_pattern.len() == 1 {
        return 0;
      }
      let mut possible_combos = 1;
      for partition_start in 1..desired_pattern.len() {
        let first_half = &desired_pattern[..partition_start];
        let second_half = &desired_pattern[partition_start..];
        let first_half_possibilities =
          get_num_possibilities(possibility_map, fails, &first_half.to_string());
        let second_half_possibilities =
          get_num_possibilities(possibility_map, fails, &second_half.to_string());
        if first_half_possibilities == 0 || second_half_possibilities == 0 {
          continue;
        }
        println!("{} += {} ({}) * {}({})", possible_combos, first_half, first_half_possibilities, second_half, second_half_possibilities);
        possible_combos *= first_half_possibilities * second_half_possibilities;
      }
      println!("There are {} ways to make {} from parts", possible_combos, desired_pattern);
      return possible_combos;
    }
  }
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let mut patterns = input[0].split(", ").collect::<Vec<&str>>();
  let mut pattern_set = HashSet::new();
  let mut fails = HashSet::new();
  patterns.sort_by(|&x, &y| x.len().cmp(&y.len()));
  let mut possible_patterns = HashMap::new();
  for &p in &patterns {
    pattern_set.insert(p.to_string());
    if p.len() == 1 {
      possible_patterns.insert(p.to_string(), 1);
    } else {
      let n = get_num_possibilities(&mut possible_patterns, &fails, &p.to_string());
      possible_patterns.insert(p.to_string(), n+1);
    }
  }

  let possible_towels = input[2..]
    .iter()
    .filter(|&pattern| {
      is_possible_design(&mut pattern_set, &mut fails, pattern)
    })
    .collect::<Vec<_>>();

  possible_towels.iter().fold(0, |acc, &pattern| {
    println!("\n{}", pattern);
    let n = get_num_possibilities(&mut possible_patterns, &fails, pattern);
    println!("{} => {}", pattern, n);
    acc + n
  })
}

#[cfg(test)]
mod day19_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      6,
      solve_part1(&inp::parse_file("test_inputs/day19_test.txt"))
    );
    assert_eq!(
      16,
      solve_part2(&inp::parse_file("test_inputs/day19_test.txt"))
    );
  }
}
