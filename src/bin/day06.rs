use std::vec::Vec;
use std::collections::HashSet;
use std::ops::Add;

const DIRECTIONS: [Point; 4] = [Point{x:0,y:-1}, Point{x:1, y:0}, Point{x: 0, y: 1}, Point{x:-1, y:0}];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
  x: i32,
  y: i32
}

impl Add for Point {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self { 
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

#[derive(PartialEq)]
enum GuardResult {
  ESCAPED,
  LOOPED
}

fn main(){
  let vec = inp::parse_file("inputs/day06.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn get_obstacles(input: &Vec<String>) -> (HashSet<Point>, Point) {
  let mut guard_position = Point{x:-1, y:-1};
  let mut obstacles = HashSet::new();
  for (y, line) in input.iter().enumerate() {
    for (x, space) in line.chars().enumerate() {
      if space == '#' {
        obstacles.insert(Point{x:x as i32, y: y as i32});
      }
      if space == '^' {
        guard_position = Point{x: x as i32, y: y as i32};
      }
    }
  }
  (obstacles, guard_position)
}

fn do_patrol(obstacles: &HashSet<Point>, start_position: &Point, grid_size: (i32, i32)) -> GuardResult {
  let mut visited = HashSet::new();
  let mut guard_direction = 0;
  let mut curr_position = *start_position;
  let mut steps_since_new_visit = 0;
  loop {
    if curr_position.x >= grid_size.0 ||
       curr_position.x < 0 ||
       curr_position.y >= grid_size.1 ||
       curr_position.y < 0 {
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

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let xsize = input[0].len() as i32;
  let ysize = input.len() as i32;
  let mut visited = HashSet::new();
  let (obstacles, mut guard_position) = get_obstacles(input);
  visited.insert(guard_position);

  let mut guard_direction = 0;
  loop {
    if guard_position.x >= xsize ||
       guard_position.x < 0 ||
       guard_position.y >= ysize ||
       guard_position.y < 0 {
      break;
    }
    visited.insert(guard_position);
    let mut next_position = guard_position + DIRECTIONS[guard_direction];
    while obstacles.contains(&next_position) {
      guard_direction = (guard_direction + 1) % DIRECTIONS.len();
      next_position = guard_position + DIRECTIONS[guard_direction];
    }
    guard_position = next_position;
  }
  visited.len() as i32

}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let xsize = input[0].len() as i32;
  let ysize = input.len() as i32;
  let (obstacles, start_position) = get_obstacles(input);
  let mut new_obstacles = obstacles.clone();
  let mut total = 0;
  for y in 0..ysize {
    for x in 0..xsize {
      println!("({},{})", x, y);
      let curr_point = Point{x:x, y:y};
      if obstacles.contains(&curr_point) || start_position == curr_point {
        continue;
      }
      new_obstacles.insert(curr_point);
      if do_patrol(&new_obstacles, &start_position, (xsize, ysize)) == GuardResult::LOOPED {
        total += 1;
      }
      new_obstacles.remove(&curr_point);
    }
  }
  total

}

#[cfg(test)]
mod day06_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(41, solve_part1(&inp::parse_file("test_inputs/day06_test.txt")));
    assert_eq!(6, solve_part2(&inp::parse_file("test_inputs/day06_test.txt")));
  }
}