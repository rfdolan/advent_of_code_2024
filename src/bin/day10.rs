use advent_of_code_2024::{inp, point::Point};
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

const RADIX: u32 = 10;

fn main() {
  let vec = inp::parse_file("inputs/day10.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn parse_input(input: &Vec<String>) -> (Vec<Point<i32>>, HashMap<Point<i32>, i32>) {
  let mut map = HashMap::new();
  let mut trailheads = Vec::new();
  for (y, line) in input.iter().enumerate() {
    for (x, height) in line
      .chars()
      .map(|x| x.to_digit(RADIX).unwrap() as i32)
      .enumerate()
    {
      let position = Point::new(x as i32, y as i32);
      map.insert(position, height);
      if height == 0 {
        trailheads.push(position);
      }
    }
  }
  (trailheads, map)
}

fn get_score(trailhead: &Point<i32>, map: &HashMap<Point<i32>, i32>) -> i32 {
  let mut score = 0;
  let mut visited = HashSet::new();
  let mut curr_node = *trailhead;
  let mut to_visit = vec![curr_node];
  loop {
    match to_visit.pop() {
      Some(node) => {
        curr_node = node;
      }
      None => {
        return score;
      }
    }
    visited.insert(curr_node);
    if let Some(height) = map.get(&curr_node) {
      if *height == 9 {
        score += 1;
      }
    }
    for neighbor in curr_node.neighbors() {
      if map.contains_key(&neighbor)
        && map[&neighbor] - map[&curr_node] == 1
        && !visited.contains(&neighbor)
      {
        to_visit.push(neighbor);
      }
    }
  }
}

fn get_score2(curr_point: &Point<i32>, map: &HashMap<Point<i32>, i32>) -> i32 {
  let mut total = 0;
  if map[&curr_point] == 9 {
    return 1;
  }
  for neighbor in curr_point.neighbors() {
    if map.contains_key(&neighbor) && map[&neighbor] - map[curr_point] == 1 {
      total += get_score2(&neighbor, map);
    }
  }
  total
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let (trailheads, map) = parse_input(input);
  let mut total = 0;
  for trailhead in trailheads {
    total += get_score(&trailhead, &map);
  }
  total
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let (trailheads, map) = parse_input(input);
  let mut total = 0;
  for trailhead in trailheads {
    total += get_score2(&trailhead, &map);
  }
  total
}

#[cfg(test)]
mod day10_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      36,
      solve_part1(&inp::parse_file("test_inputs/day10_test.txt"))
    );
    assert_eq!(
      81,
      solve_part2(&inp::parse_file("test_inputs/day10_test.txt"))
    );
  }
}
