use std::vec::Vec;
use std::ops::{Sub,Add};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

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

fn get_antennas(input: &Vec<String>) -> HashMap<char, Vec<Point>> {
  let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
  for (y, line) in input.iter().enumerate() {
    for (x, character) in line.chars().enumerate() {
      if character == '.' { continue;}
      let point = Point{x: x as i32, y: y as i32};
      match antennas.entry(character) {
        Entry::Occupied(mut entry) => entry.get_mut().push(point),
        Entry::Vacant(entry) => {
            entry.insert(vec![point]);
        }
      }
    }
  }
  antennas
}

fn create_antinodes(antennas: &Vec<Point>, anodes: &mut HashSet<Point>, gridsize: (i32, i32))  {
  for (i1, &point1) in antennas.iter().enumerate(){
    for &point2 in &antennas[i1+1..]{
      let diff = point1 - point2;
      let spots = vec![point1+diff, point2+diff, point1-diff, point2-diff];
      for spot in spots {
        if spot != point1 && spot != point2 &&
        (0..gridsize.0).contains(&spot.x) &&
        (0..gridsize.1).contains(&spot.y) {
          anodes.insert(spot);
        }
      }
    }
  }
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let antennas = get_antennas(input);
  let xsize = input[0].len() as i32;
  let ysize = input.len() as i32;
  let mut anodes = HashSet::new();
  for (_label, locations) in antennas {
    create_antinodes(&locations, &mut anodes, (xsize, ysize));
  }
  anodes.len() as i32
}

fn create_antinodes2(antennas: &Vec<Point>, anodes: &mut HashSet<Point>, gridsize: (i32, i32))  {
  for (i1, &point1) in antennas.iter().enumerate(){
    for &point2 in &antennas[i1+1..]{
      let diff = point1 - point2;
      let mut it = point1-diff;
      while (0..gridsize.0).contains(&it.x) && (0..gridsize.1).contains(&it.y) {
        anodes.insert(it);
        it = it - diff;
      }
      it = point1+diff;
      while (0..gridsize.0).contains(&it.x) && (0..gridsize.1).contains(&it.y) {
        anodes.insert(it);
        it = it + diff;
      }
      anodes.insert(point1);
    }
  }
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let antennas = get_antennas(input);
  let xsize = input[0].len() as i32;
  let ysize = input.len() as i32;
  let mut anodes = HashSet::new();
  for (_, locations) in antennas {
    create_antinodes2(&locations, &mut anodes, (xsize, ysize));
  }
  anodes.len() as i32
}

#[cfg(test)]
mod day08_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(14, solve_part1(&inp::parse_file("test_inputs/day08_test.txt")));
    assert_eq!(34, solve_part2(&inp::parse_file("test_inputs/day08_test.txt")));
  }
}