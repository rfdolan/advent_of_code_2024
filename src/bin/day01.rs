use advent_of_code_2024::inp;
use std::collections::HashMap;
use std::vec::Vec;

fn main() {
  let vec = inp::parse_file("inputs/day01.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let (mut left_nums, mut right_nums) = make_lists(input);
  left_nums.sort();
  right_nums.sort();

  let mut difference = 0;
  for x in 0..left_nums.len() {
    difference += (left_nums[x] - right_nums[x]).abs();
  }
  difference
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let (left_nums, right_nums) = make_lists(input);
  let mut list_2_map: HashMap<i32, i32> = HashMap::new();

  for item in right_nums {
    let val = list_2_map.entry(item).or_insert(0);
    *val += 1;
  }

  let mut similarity_score = 0;
  for item in left_nums {
    match list_2_map.get(&item) {
      Some(val) => similarity_score += item * val,
      None => (),
    }
  }
  similarity_score
}

fn make_lists(input: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
  let mut left_nums: Vec<i32> = Vec::new();
  let mut right_nums: Vec<i32> = Vec::new();
  for pair in input {
    let pair = pair.split(" ").collect::<Vec<_>>();
    left_nums.push(pair[0].parse::<i32>().unwrap());
    right_nums.push(pair[pair.len() - 1].parse::<i32>().unwrap());
  }
  (left_nums, right_nums)
}

#[cfg(test)]
mod day01_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      11,
      solve_part1(&inp::parse_file("test_inputs/day01_test.txt"))
    );
    assert_eq!(
      31,
      solve_part2(&inp::parse_file("test_inputs/day01_test.txt"))
    );
  }
}
