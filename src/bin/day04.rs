use advent_of_code_2024::{inp, point::Point};
use std::vec::Vec;
use std::collections::HashMap;

const DIRECTIONS: [Point; 8] = [Point{x:0,y:1},Point{x: 0, y: -1},Point{x: 1, y: 1},Point{x: 1, y: 0},Point{x: 1, y: -1},Point{x: -1, y: 1},Point{x: -1, y: 0},Point{x: -1, y: -1}];
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn main(){
  let vec = inp::parse_file("inputs/day04.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn build_map(input: &Vec<String>) -> (HashMap<Point, char>, usize) {
  let mut map = HashMap::new();
  let mut xsize = 0;
  for (y, line) in input.iter().enumerate() {
    for (x, character) in line.chars().enumerate() {
      map.insert(Point::new( x as i32,  y as i32), character);
      if x > xsize {
        xsize = x+1;
      }
    }
  }
  (map, xsize)
}

fn find_xmas_in_direction(map: &HashMap<Point, char>, start: &Point, direction: &Point) -> bool {
  let mut curr_point = *start + *direction;
  for letter in 1..XMAS.len() {
    match map.get(&curr_point) {
      Some(character) => {
        if *character != XMAS[letter] {
          return false;
        }
      },
      None => return false
    }
    curr_point = curr_point + *direction;
  }
  return true;
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let (map, xsize) = build_map(input);

  let mut total_xmas = 0;
  for y in 0..input.len() {
    for x in 0..xsize {
      let curr_point = Point::new(x as i32, y as i32);
      if *map.get(&curr_point).unwrap() == 'X' {
        for direction in DIRECTIONS {
          if find_xmas_in_direction(&map, &curr_point, &direction) {
            total_xmas += 1;
          }
        }
      }
    }
  }
  total_xmas
}

fn is_x_mas(map: &HashMap<Point, char>, start: &Point) -> Option<bool> {
  let upleft = map.get(&(*start + Point::new( -1, -1))) ?;
  let upright = map.get(&(*start + Point::new( 1,-1))) ?;
  let downleft = map.get(&(*start + Point::new( -1,1))) ?;
  let downright = map.get(&(*start + Point::new( 1,1))) ?;
  let mut diag1_good = false;
  let mut diag2_good = false;
  if (*upleft == 'M' && *downright == 'S') || (*upleft == 'S' && *downright == 'M') {
    diag1_good = true;
  }
  if (*upright == 'M' && *downleft == 'S') || (*upright == 'S' && *downleft == 'M') {
    diag2_good = true;
  }

  Some(diag1_good && diag2_good)
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let (map, xsize) = build_map(input);

  let mut total_x_mas = 0;
  for y in 0..input.len() {
    for x in 0..xsize {
      let curr_point = Point::new(x as i32,y as i32);
      if *map.get(&curr_point).unwrap() == 'A' {
       match is_x_mas(&map, &curr_point) {
        Some(found) => {
          if found {
            total_x_mas += 1;
          }
        },
        None => ()
       }
      }
    }
  }
  total_x_mas
}

#[cfg(test)]
mod day04_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(18, solve_part1(&inp::parse_file("test_inputs/day04_test.txt")));
    assert_eq!(9, solve_part2(&inp::parse_file("test_inputs/day04_test.txt")));
  }
}