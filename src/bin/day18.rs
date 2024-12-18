use advent_of_code_2024::{inp, point::Point};
use std::vec::Vec;
use std::collections::{HashSet,HashMap,VecDeque};
use itertools::Itertools;

const TEST_CUTOFF: usize = 12;
const TEST_SIZE: (usize, usize) = (6,6);
const CUTOFF: usize = 1024;
const SIZE: (usize, usize) = (70,70);

fn main() {
  let vec = inp::parse_file("inputs/day18.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec, CUTOFF, SIZE));
  println!("Part 2: {:?}", solve_part2(&vec, CUTOFF, SIZE));
}

fn dijkstras(start: &Point<i32>, corrupted: &HashSet<Point<i32>>, size: (usize, usize)) -> HashMap<Point<i32>, i32> {
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
      if !(0..size.0+1).contains(&(neighbor.x as usize)) 
        || !(0..size.1+1).contains(&(neighbor.y as usize)) 
        || corrupted.contains(&neighbor) {
        continue;
      }
      match distances.get(&neighbor) {
        Some(&next_dist) => {
          if curr_dist + 1 < next_dist {
            distances.insert(neighbor, curr_dist + 1);
            to_visit.push_back(neighbor);
          }
        },
        None => {
          distances.insert(neighbor, curr_dist + 1);
          to_visit.push_back(neighbor);
        }
      }
    }
  }
  distances
}

/*fn back_trace(start: &Point<i32>, end: &Point<i32>, min_tree: &HashMap<Point<i32>, i32>) -> Vec<Point<i32>> {
  let mut curr_dist = min_tree[start];
  let mut curr_point = *start;
  while curr_point != *end {
  }

}
  */

// Solution for part 1
fn solve_part1(input: &Vec<String>, cutoff: usize, size: (usize, usize)) -> i32 {
  let mut bytes = HashSet::new();
  for (i, line) in input.iter().enumerate() {
    if i >=cutoff {
      break;
    }
    let (x,y) = line.split(",").map(|x| x.parse::<i32>().unwrap()).next_tuple().unwrap();
    bytes.insert(Point::new(x, y));
  }

  let d = dijkstras(&Point::new(0 as i32,0 as i32),&bytes, size);
  d[&Point::new(size.0 as i32,size.1 as i32)] 
}

// Solution for part 2
fn solve_part2(input: &Vec<String>, cutoff: usize, size: (usize, usize)) -> (i32, i32) {
  let mut bytes = HashSet::new();
  for (i, line) in input.iter().enumerate() {
    if i >=cutoff {
      break;
    }
    let (x,y) = line.split(",").map(|x| x.parse::<i32>().unwrap()).next_tuple().unwrap();
    bytes.insert(Point::new(x, y));
  }

  let start: Point<i32> = Point::zero();
  let end = Point::new(size.0 as i32, size.1 as i32);
  for next_byte in &input[cutoff..] {
    let (x,y) = next_byte.split(",").map(|x| x.parse::<i32>().unwrap()).next_tuple().unwrap();
    bytes.insert(Point::new(x, y));
    let d = dijkstras(&start,&bytes, size);
    match d.get(&end) {
      Some(_) => (),
      None => {
        return (x,y);
      }
    }
  }
  (-1,-1)
}

#[cfg(test)]
mod day18_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(22, solve_part1(&inp::parse_file("test_inputs/day18_test.txt"), TEST_CUTOFF, TEST_SIZE));
    assert_eq!((6,1), solve_part2(&inp::parse_file("test_inputs/day18_test.txt"), TEST_CUTOFF, TEST_SIZE));
  }
}
