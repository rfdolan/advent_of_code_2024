use advent_of_code_2024::inp;
use std::collections::HashMap;
use std::vec::Vec;

const TURNS1: usize = 25;
const TURNS2: usize = 75;

fn main() {
  let vec = inp::parse_file("inputs/day11.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn rule(initial_stones: &Vec<i64>, turns: usize) -> i64 {
  let mut stones = HashMap::new();
  for &stone in initial_stones {
    stones.insert(stone, 1);
  }
  for _turn in 0..turns {
    let mut next_stones = HashMap::new();
    for (stone, val) in stones {
      if stone == 0 {
        next_stones
          .entry(1)
          .and_modify(|x| *x += val)
          .or_insert(val);
        continue;
      }
      let stone_string = stone.to_string();
      if stone_string.len() % 2 == 0 {
        next_stones
          .entry(
            stone_string[0..stone_string.len() / 2]
              .parse::<i64>()
              .unwrap(),
          )
          .and_modify(|x| *x += val)
          .or_insert(val);
        next_stones
          .entry(
            stone_string[stone_string.len() / 2..]
              .parse::<i64>()
              .unwrap(),
          )
          .and_modify(|x| *x += val)
          .or_insert(val);
      } else {
        next_stones
          .entry(stone * 2024)
          .and_modify(|x| *x += val)
          .or_insert(val);
      }
    }
    stones = next_stones;
  }
  stones.iter().fold(0, |total, (_, val)| total + val)
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i64 {
  let stones = input[0]
    .split(" ")
    .map(|x| x.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();
  rule(&stones, TURNS1)
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i64 {
  let stones = input[0]
    .split(" ")
    .map(|x| x.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();
  rule(&stones, TURNS2)
}

#[cfg(test)]
mod day11_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      55312,
      solve_part1(&inp::parse_file("test_inputs/day11_test.txt"))
    );
    assert_eq!(
      55312,
      solve_part2(&inp::parse_file("test_inputs/day11_test.txt"))
    );
  }
}
