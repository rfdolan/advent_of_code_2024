use advent_of_code_2024::{inp, point::Point};
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

fn main() {
  let vec = inp::parse_file("inputs/day12.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn get_region(
  point: char,
  start: &Point<i32>,
  all_points: &HashMap<Point<i32>, char>,
) -> HashSet<Point<i32>> {
  let mut region = HashSet::new();
  let mut to_check = Vec::new();
  to_check.push(start.clone());
  while !to_check.is_empty() {
    let current = to_check.pop().unwrap();
    if all_points.contains_key(&current)
      && all_points.get(&current).unwrap() == &point
    {
      region.insert(current.clone());
      for neighbor in current.neighbors() {
        if !region.contains(&neighbor) {
          to_check.push(neighbor);
        }
      }
    }
  }
  region
}

fn get_perimeter(region: &HashSet<Point<i32>>) -> i32 {
  let mut perimeter = 0;
  for point in region {
    let mut num_neighbors = 0;
    for neighbor in point.neighbors() {
      if region.contains(&neighbor) {
        num_neighbors += 1;
      }
    }
    perimeter += 4 - num_neighbors;
  }
  perimeter
}

fn get_all_regions(input: &Vec<String>) -> Vec<HashSet<Point<i32>>> {
  // Parse data
  let mut all_points = HashMap::new();
  for (y, line) in input.iter().enumerate() {
    for (x, c) in line.chars().enumerate() {
      all_points.insert(Point::new(x as i32, y as i32), c);
    }
  }

  // Collect into a vector of regions
  let all_points = all_points;
  let mut regions = Vec::new();
  let mut accounted_for: HashSet<Point<i32>> = HashSet::new();
  for point in &all_points {
    if !accounted_for.contains(&point.0) {
      let region = get_region(*point.1, &point.0, &all_points);
      accounted_for.extend(region.iter());
      regions.push(region);
    }
  }
  regions
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let regions = get_all_regions(input);

  regions.iter().fold(0, |acc, region| {
    let area = region.len() as i32;
    let perimeter = get_perimeter(&region);
    acc + area * perimeter
  })
}

// The number of sides is equal to the number of corners, which are easier to
// find.
fn get_num_sides(region: &HashSet<Point<i32>>) -> i32 {
  let start = region.iter().next().unwrap().clone();
  let mut corners = 0;
  let mut to_visit = Vec::with_capacity(region.len());
  let mut visited = HashSet::with_capacity(region.len());
  to_visit.push(start);
  visited.insert(start);

  while let Some(curr) = to_visit.pop() {
    let neighbors: Vec<_> = curr
      .neighbors()
      .into_iter()
      .filter(|x| region.contains(x))
      .collect();

    // We are alone in this cold, cruel world...
    if neighbors.is_empty() {
      return 4;
    }

    for &neighbor in neighbors.iter() {
      if visited.insert(neighbor) {
        to_visit.push(neighbor);
      }
    }

    // End point, add 2 for its corners.
    // ex.
    // +---+
    // | A |
    // | A |
    if neighbors.len() == 1 {
      corners += 2;
      continue;
    }
    if neighbors.len() == 2 {
      // Don't add anything for a straight line
      // ex.
      // -------
      //  A A A
      // -------
      if neighbors[0].x == neighbors[1].x || neighbors[0].y == neighbors[1].y {
        continue;
      }
      // We have an exterior corner
      // ex.
      // +-----
      // | A A
      // | A ?
      corners += 1;
    }

    // Check interior corners and t-bars
    for (i, n1) in neighbors.iter().enumerate() {
      for n2 in neighbors[i..].iter() {
        // Finding interior corners
        // ----------
        // A   A   A
        // --+   +--
        //   | A |
        for point in [Point::new(n1.x, n2.y), Point::new(n2.x, n1.y)] {
          if point != curr && !region.contains(&point) {
            corners += 1;
          }
        }
      }
    }
  }
  corners
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let regions = get_all_regions(input);

  regions.iter().fold(0, |acc, region| {
    let area = region.len() as i32;
    let num_sides = get_num_sides(&region);
    acc + area * num_sides
  })
}

#[cfg(test)]
mod day12_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      140,
      solve_part1(&inp::parse_file("test_inputs/day12_test1.txt"))
    );
    assert_eq!(
      772,
      solve_part1(&inp::parse_file("test_inputs/day12_test2.txt"))
    );
    assert_eq!(
      1930,
      solve_part1(&inp::parse_file("test_inputs/day12_test3.txt"))
    );
    assert_eq!(
      80,
      solve_part2(&inp::parse_file("test_inputs/day12_test1.txt"))
    );
    assert_eq!(
      436,
      solve_part2(&inp::parse_file("test_inputs/day12_test2.txt"))
    );
    assert_eq!(
      1206,
      solve_part2(&inp::parse_file("test_inputs/day12_test3.txt"))
    );
    assert_eq!(
      236,
      solve_part2(&inp::parse_file("test_inputs/day12_test4.txt"))
    );
    assert_eq!(
      368,
      solve_part2(&inp::parse_file("test_inputs/day12_test5.txt"))
    );
  }
}
