use advent_of_code_2024::inp;
use std::vec::Vec;

fn main() {
  let vec = inp::parse_file("inputs/dayxxx.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

// Solution for part 1
fn solve_part1(_input: &Vec<String>) -> i32 {
  0
}

// Solution for part 2
fn solve_part2(_input: &Vec<String>) -> i32 {
  0
}

#[cfg(test)]
mod dayxxx_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      0,
      solve_part1(&inp::parse_file("test_inputs/dayxxx_test.txt"))
    );
  }
}
