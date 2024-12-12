use advent_of_code_2024::inp;
use std::vec::Vec;

fn main() {
  let vec = inp::parse_file("inputs/day02.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn is_safe(report: &Vec<i32>) -> bool {
  let mut is_increasing = false;
  let mut is_decreasing = false;
  let mut prev = report[0];
  for &item in &report[1..] {
    let diff = item - prev;
    if diff == 0 {
      return false;
    }
    if diff > 0 {
      is_increasing = true;
    }
    if diff < 0 {
      is_decreasing = true;
    }
    if is_decreasing && is_increasing {
      return false;
    }
    if diff.abs() > 3 {
      return false;
    }
    prev = item;
  }
  return true;
}

fn problem_damper(report: &Vec<i32>) -> bool {
  if is_safe(report) {
    return true;
  }
  for skipped_index in 0..report.len() {
    let mut subset = Vec::new();
    for (x, &item) in report.iter().enumerate() {
      if x != skipped_index {
        subset.push(item);
      }
    }
    if is_safe(&subset) {
      return true;
    }
  }
  return false;
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let mut safe_lines = 0;
  for line in input {
    let line = line
      .split(" ")
      .map(|x| x.parse::<i32>().unwrap())
      .collect::<Vec<i32>>();
    if is_safe(&line) {
      safe_lines += 1;
    }
  }
  safe_lines
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let mut safe_lines = 0;
  for line in input {
    let line = line
      .split(" ")
      .map(|x| x.parse::<i32>().unwrap())
      .collect::<Vec<i32>>();
    if problem_damper(&line) {
      safe_lines += 1;
    }
  }
  safe_lines
}

#[cfg(test)]
mod day02_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      2,
      solve_part1(&inp::parse_file("test_inputs/day02_test.txt"))
    );
    assert_eq!(
      4,
      solve_part2(&inp::parse_file("test_inputs/day02_test.txt"))
    );
  }
}
