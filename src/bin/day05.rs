use std::vec::Vec;
use itertools::Itertools;

fn main(){
  let vec = inp::parse_file("inputs/day05.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn is_correct_order(rules: &Vec<(i32, i32)>, report: &Vec<i32>) -> bool {
  for (first, second) in rules {
    let first_index = report.iter().position(|&x| x == *first);
    let second_index = report.iter().position(|&x| x == *second);
    if first_index.is_none() || second_index.is_none() {
      continue;
    }
    if first_index > second_index {
      return false;
    }
  }
  true
}

fn get_rules(input: &Vec<String>) -> Vec<(i32, i32)> {
  let mut rules: Vec<(i32, i32)>  = Vec::new();
  for line in input {
    if line == "" { return rules;}
    rules.push(line.split("|").map(|x| x.parse::<i32>().unwrap()).next_tuple().unwrap());
  }
  rules
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let rules = get_rules(input);
  let mut total = 0;
  for report in &input[rules.len()+1..] {
    let report = report.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    if is_correct_order(&rules, &report) {
      total += report[report.len()/2];
    }
  }
  total
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let rules = get_rules(input);
  let mut total = 0;
  for report in &input[rules.len()+1..] {
    let mut report = report.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    if !is_correct_order(&rules, &report) {
      while !is_correct_order(&rules, &report) {
        for (first, second) in &rules {
          let first_index = report.iter().position(|&x| x == *first);
          let second_index = report.iter().position(|&x| x == *second);
          if !first_index.is_none() && !second_index.is_none() && first_index > second_index {
            report.swap(first_index.unwrap(), second_index.unwrap());
          }
        }
      }
      total += report[report.len()/2];
    }
  }
  total
}

#[cfg(test)]
mod day05_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(143, solve_part1(&inp::parse_file("test_inputs/day05_test.txt")));
    assert_eq!(123, solve_part2(&inp::parse_file("test_inputs/day05_test.txt")));
  }
}