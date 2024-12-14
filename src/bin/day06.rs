use advent_of_code_2024::{inp, point::Point};
use std::collections::HashSet;
use std::thread;
use std::vec::Vec;

const DIRECTIONS: [Point<i32>; 4] = [
  Point { x: 0, y: -1 },
  Point { x: 1, y: 0 },
  Point { x: 0, y: 1 },
  Point { x: -1, y: 0 },
];

#[derive(PartialEq)]
enum GuardResult {
  ESCAPED,
  LOOPED,
}

fn main() {
  let vec = inp::parse_file("inputs/day06.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn get_obstacles(input: &Vec<String>) -> (HashSet<Point<i32>>, Point<i32>) {
  let mut guard_position = Point::new(-1, -1);
  let mut obstacles = HashSet::new();
  for (y, line) in input.iter().enumerate() {
    for (x, space) in line.chars().enumerate() {
      if space == '#' {
        obstacles.insert(Point::new(x as i32, y as i32));
      }
      if space == '^' {
        guard_position = Point::new(x as i32, y as i32);
      }
    }
  }
  (obstacles, guard_position)
}

fn do_patrol(
  obstacles: &HashSet<Point<i32>>,
  start_position: &Point<i32>,
  grid_size: (i32, i32),
) -> GuardResult {
  let mut visited = HashSet::new();
  let mut guard_direction = 0;
  let mut curr_position = *start_position;
  let mut steps_since_new_visit = 0;
  loop {
    if curr_position.x >= grid_size.0
      || curr_position.x < 0
      || curr_position.y >= grid_size.1
      || curr_position.y < 0
    {
      return GuardResult::ESCAPED;
    }
    if !visited.contains(&curr_position) {
      visited.insert(curr_position);
      steps_since_new_visit = 0;
    } else {
      steps_since_new_visit += 1;
    }
    if steps_since_new_visit > std::cmp::max(grid_size.0, grid_size.1) {
      return GuardResult::LOOPED;
    }
    let mut next_position = curr_position + DIRECTIONS[guard_direction];
    while obstacles.contains(&next_position) {
      guard_direction = (guard_direction + 1) % DIRECTIONS.len();
      next_position = curr_position + DIRECTIONS[guard_direction];
    }
    curr_position = next_position;
  }
}

fn get_path(
  obstacles: &HashSet<Point<i32>>,
  size: (i32, i32),
  guard_position: &Point<i32>,
) -> HashSet<Point<i32>> {
  let mut visited = HashSet::new();
  let mut guard_position = *guard_position;
  visited.insert(guard_position);

  let mut guard_direction = 0;
  loop {
    if guard_position.x >= size.0
      || guard_position.x < 0
      || guard_position.y >= size.1
      || guard_position.y < 0
    {
      return visited;
    }
    visited.insert(guard_position);
    let mut next_position = guard_position + DIRECTIONS[guard_direction];
    while obstacles.contains(&next_position) {
      guard_direction = (guard_direction + 1) % DIRECTIONS.len();
      next_position = guard_position + DIRECTIONS[guard_direction];
    }
    guard_position = next_position;
  }
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let xsize = input[0].len() as i32;
  let ysize = input.len() as i32;
  let (obstacles, guard_position) = get_obstacles(input);

  get_path(&obstacles, (xsize, ysize), &guard_position).len() as i32
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let xsize = input[0].len() as i32;
  let ysize = input.len() as i32;
  let (obstacles, start_position) = get_obstacles(input);
  let visited = get_path(&obstacles, (xsize, ysize), &start_position);
  let mut total = 0;
  let mut children = vec![];
  for cell in visited {
    let mut new_obstacles = obstacles.clone();
    if start_position == cell {
      continue;
    }
    new_obstacles.insert(cell);
    children.push(thread::spawn(move || {
      do_patrol(&new_obstacles, &start_position, (xsize, ysize))
    }));
  }
  for child in children {
    if child.join().unwrap() == GuardResult::LOOPED {
      total += 1;
    }
  }
  total
}

#[cfg(test)]
mod day06_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      41,
      solve_part1(&inp::parse_file("test_inputs/day06_test.txt"))
    );
    assert_eq!(
      6,
      solve_part2(&inp::parse_file("test_inputs/day06_test.txt"))
    );
  }
}
