use std::vec::Vec;
use std::ops::{Sub,Add};
use std::collections::{HashMap, HashSet};

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

impl Sub for Point {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self { 
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

fn main(){
  let vec = inp::parse_file("inputs/day08.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn create_antinodes(antennas: &Vec<Point>, anodes: &mut HashSet<Point>, gridsize: (i32, i32))  {
  for a1 in 0..antennas.len() {
    let point1 = antennas[a1];
    for a2 in a1..antennas.len() {
      let point2= antennas[a2];
      let diff = point1 - point2;
      let spots = vec![point1+diff, point2+diff, point1-diff, point2-diff];
      for spot in spots {
        if spot != point1 && spot != point2 &&
        !anodes.contains(&spot) && 
        (0..gridsize.0).contains(&spot.x) &&
        (0..gridsize.1).contains(&spot.y) {
          anodes.insert(spot);
          //println!("{:?}", spot);
        }
      }
    }
  }
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
  let mut is_occupied = HashSet::new();
  let xsize = input[0].len() as i32;
  let ysize = input.len() as i32;
  for (y, line) in input.iter().enumerate() {
    for (x, character) in line.chars().enumerate() {
      if character == '.' { continue;}
      let point = Point{x: x as i32, y: y as i32};
      match antennas.get_mut(&character) {
        Some(val) => val.push(point),
        None => {
          antennas.insert(character, vec![point]);
          ()}
      }
      is_occupied.insert(point);
    }
  }
  let mut anodes = HashSet::new();
  for antenna_type in antennas {
    create_antinodes(&antenna_type.1, &mut anodes, (xsize, ysize));

  }
  anodes.len() as i32
}

// Solution for part 2
fn solve_part2(_input: &Vec<String>) -> i32 {
  0
}

#[cfg(test)]
mod day08_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(14, solve_part1(&inp::parse_file("test_inputs/day08_test.txt")));
  }
}