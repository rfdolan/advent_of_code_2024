use advent_of_code_2024::{inp, point::{Point, CARDINAL_DIRS}};
use std::vec::Vec;
use std::collections::{HashSet, HashMap, VecDeque};

const TEST_TARGET: i32 = 50;
const TARGET: i32 = 100;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cheat {
  start: Point<i32>,
  end: Point<i32>,
  savings: i32
}

fn main() {
  let vec = inp::parse_file("inputs/day20.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec, TARGET));
  println!("Part 2: {}", solve_part2(&vec, TARGET));
}

fn dijkstras(
  start: &Point<i32>,
  walls: &HashSet<Point<i32>>,
  size: (usize, usize),
) -> HashMap<Point<i32>, i32> {
  let mut distances = HashMap::new();
  let mut to_visit = VecDeque::new();
  // Initialize all distances to infinite
  for y in 0..size.1 {
    for x in 0..size.0 {
      distances.insert(Point::new(x as i32, y as i32), std::i32::MAX);
    }
  }
  distances.insert(*start, 0);
  to_visit.push_back(*start);
  while let Some(current) = to_visit.pop_front() {
    let curr_dist = distances[&current];
    for neighbor in current.neighbors() {
      if !(0..size.0 + 1).contains(&(neighbor.x as usize))
        || !(0..size.1 + 1).contains(&(neighbor.y as usize))
        || walls.contains(&neighbor)
      {
        continue;
      }
      match distances.get(&neighbor) {
        Some(&next_dist) => {
          if curr_dist + 1 < next_dist {
            distances.insert(neighbor, curr_dist + 1);
            to_visit.push_back(neighbor);
          }
        }
        None => {
          distances.insert(neighbor, curr_dist + 1);
          to_visit.push_back(neighbor);
        }
      }
    }
  }
  distances
}

fn try_cheat_from(start_position: &Point<i32>, shortest_paths: &HashMap<Point<i32>, i32>) -> Vec<Cheat> {
  let mut cheats = Vec::new();
  let start_cost = shortest_paths.get(start_position).unwrap();
  if *start_cost > std::i32::MAX - 10 {
    return cheats;
  }
  for dir in CARDINAL_DIRS {
    //let first = *start_position + dir;
    let second = *start_position + dir + dir;
    match shortest_paths.get(&second) {
      Some(&cheat_result) => {
        if cheat_result < start_cost - 2 {
          cheats.push(Cheat{start: *start_position, end: second, savings: start_cost - 2 - cheat_result});
        }
      },
      None => ()
    }
  }
  cheats
}


fn try_cheat_from2(start_position: &Point<i32>, shortest_paths: &HashMap<Point<i32>, i32>) -> HashSet<Cheat> {
  let mut cheats = HashSet::new();
  let start_cost = shortest_paths.get(start_position).unwrap();
  if *start_cost > std::i32::MAX - 10 {
    return cheats;
  }
  for x_diff in -20 as i32..21 as i32 {
    for y_diff in -20 as i32..21 as i32 {
      if x_diff.abs() + y_diff.abs() > 20 {
        continue;
      }
      let target = *start_position + Point::new(x_diff, y_diff);
      let dist = start_position.manhattan_dist(target);
      match shortest_paths.get(&target) {
        Some(&cheat_result) => {
          if cheat_result < start_cost - dist {
            cheats.insert(Cheat{start: *start_position, end: target, savings: start_cost - dist - cheat_result});
          }
        },
        None => ()
      }
    }
  }
  cheats
}

// Solution for part 1
fn solve_part1(input: &Vec<String>, more_than: i32) -> i32 {
  let mut walls = HashSet::new();
  let mut start: Point<i32> = Point::zero();
  let mut end: Point<i32> = Point::zero();
  let size = (input.len(), input[0].len());
  for (y, line) in input.iter().enumerate() {
    for (x, val) in line.chars().enumerate() {
      match val {
        '#' => {walls.insert(Point::new(x as i32,y as i32));},
        'S' => {start = Point::new(x as i32,y as i32);},
        'E' => {end = Point::new(x as i32,y as i32);},
        _ => ()
      }
    }
  }
  let d = dijkstras(&end, &walls, size);
  let mut cheats = Vec::new();
  for start in d.keys() {
    cheats.append(&mut try_cheat_from(start, &d));
  }

  cheats.iter().fold(0, |mut acc, cheat| {
    if cheat.savings >=more_than {
      acc += 1;
    } 
    acc
  })
}

// Solution for part 2
fn solve_part2(input: &Vec<String>, more_than: i32) -> i32 {
  let mut walls = HashSet::new();
  let mut start: Point<i32> = Point::zero();
  let mut end: Point<i32> = Point::zero();
  let size = (input.len(), input[0].len());
  for (y, line) in input.iter().enumerate() {
    for (x, val) in line.chars().enumerate() {
      match val {
        '#' => {walls.insert(Point::new(x as i32,y as i32));},
        'S' => {start = Point::new(x as i32,y as i32);},
        'E' => {end = Point::new(x as i32,y as i32);},
        _ => ()
      }
    }
  }
  let d = dijkstras(&end, &walls, size);
  let mut cheats = HashSet::new();
  for start in d.keys() {
    cheats.extend(try_cheat_from2(start, &d));
  }

  cheats.iter().fold(0, |mut acc, cheat| {
    if cheat.savings >=more_than {
      acc += 1;
    } 
    acc
  })
}

#[cfg(test)]
mod day20_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      5,
      solve_part1(&inp::parse_file("test_inputs/day20_test.txt"), 20)
    );
    assert_eq!(
      285,
      solve_part2(&inp::parse_file("test_inputs/day20_test.txt"), TEST_TARGET)
    );
  }
}
