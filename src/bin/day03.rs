use std::vec::Vec;

fn main(){
  let vec = inp::parse_file("inputs/day03.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn step_through_after_mul(line: &str) -> i32 {
  let characters = line.chars().collect::<Vec<char>>();
  if characters[0] != '(' {
    return 0;
  }
  let mut finished_num1 = false;
  let mut num1_string: Vec<char> = Vec::new();
  let mut num1 = 0;
  let mut num2_string: Vec<char> = Vec::new();
  for &character in &characters[1..] {
    if !character.is_numeric() {
      if character == ',' {
        if finished_num1 { return 0; }
        match num1_string.iter().collect::<String>().parse::<i32>() {
          Ok(value) => num1 = value,
          _=> return 0 
        }
        finished_num1 = true;
      } else if character == ')' {
        if !finished_num1 { return 0; }
        match num2_string.iter().collect::<String>().parse::<i32>() {
          Ok(value) => return num1 * value,
          _=> return 0 
        }
      } else {
        return 0;
      }
    } else {
      if !finished_num1 {
        num1_string.push(character);
      } else {
        num2_string.push(character);
      }
    }
  }
  return 0;
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let mut total = 0;
  for line in input {
    let bits = line.split("mul").collect::<Vec<_>>();
    for instruction in bits {
      total += step_through_after_mul(&instruction);
    }
  }
  total
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let mut total = 0;
  let mut enabled = true;
  for line in input {
    let bits = line.split("mul").collect::<Vec<_>>();
    for instruction in bits {
      if enabled {
        total += step_through_after_mul(&instruction);
      }
      let dosplit = instruction.split("do()").last().unwrap();
      let dontsplit = instruction.split("don't()").last().unwrap();
      if dosplit.len() != dontsplit.len() {
        if dosplit.len() < dontsplit.len() {
          enabled = true;
        } else {
          enabled = false;
        }
      }
    }
  }
  total
}

#[cfg(test)]
mod day03_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(161, solve_part1(&inp::parse_file("test_inputs/day03_test1.txt")));
    assert_eq!(48, solve_part2(&inp::parse_file("test_inputs/day03_test2.txt")));
  }
}