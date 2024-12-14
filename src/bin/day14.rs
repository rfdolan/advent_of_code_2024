use advent_of_code_2024::{inp, point::Point};
use regex::Regex;
use std::fs::File;
use std::io::LineWriter;
use std::ops::Range;
use std::vec::Vec;

const GRIDSIZE: Point<i32> = Point { x: 101, y: 103 };
const PART1_TURNS: i32 = 100;

#[derive(Debug)]
struct Robot {
  pos: Point<i32>,
  vel: Point<i32>,
}

fn main() {
  let vec = inp::parse_file("inputs/day14.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec, GRIDSIZE));
  println!("Part 2: {}", solve_part2(&vec, GRIDSIZE));
}

fn get_robots_in_quadrant(
  robots: &Vec<Robot>,
  x_range: Range<i32>,
  y_range: Range<i32>,
) -> i32 {
  let mut count = 0;
  for robot in robots {
    if x_range.contains(&robot.pos.x) && y_range.contains(&robot.pos.y) {
      count += 1;
    }
  }
  count
}

// Solution for part 1
fn solve_part1(input: &Vec<String>, gridsize: Point<i32>) -> i32 {
  let mut robots = Vec::new();
  let re = Regex::new(r"(-?\d+)").unwrap();
  for line in input {
    let captures: Vec<i32> = re
      .find_iter(line)
      .map(|m| m.as_str().parse::<i32>().unwrap())
      .collect();
    robots.push(Robot {
      pos: Point::new(captures[0], captures[1]),
      vel: Point::new(captures[2], captures[3]),
    });
  }

  for x in 0..PART1_TURNS {
    for robot in robots.iter_mut() {
      robot.pos = Point::new(
        (robot.pos.x + robot.vel.x).rem_euclid(gridsize.x),
        (robot.pos.y + robot.vel.y).rem_euclid(gridsize.y),
      );
    }
  }
  get_robots_in_quadrant(&robots, 0..gridsize.x / 2, 0..gridsize.y / 2)
    * get_robots_in_quadrant(
      &robots,
      gridsize.x / 2 + 1..gridsize.x,
      0..gridsize.y / 2,
    )
    * get_robots_in_quadrant(
      &robots,
      0..gridsize.x / 2,
      gridsize.y / 2 + 1..gridsize.y,
    )
    * get_robots_in_quadrant(
      &robots,
      gridsize.x / 2 + 1..gridsize.x,
      gridsize.y / 2 + 1..gridsize.y,
    )
}

// Prints to the console or can also print to a file if you like.
fn print_grid(
  robots: &Vec<Robot>,
  gridsize: Point<i32>,
  _file: &mut LineWriter<File>,
) {
  let mut grid = vec![vec!['.'; gridsize.x as usize]; gridsize.y as usize];
  for robot in robots {
    grid[robot.pos.y as usize][robot.pos.x as usize] = '#';
  }
  for row in grid {
    //file.write_all((row.iter().collect::<String>() + "\n").as_bytes()).expect("Failed to write to file!");
    println!("{}", row.iter().collect::<String>());
  }
}

// Solution for part 2
fn solve_part2(input: &Vec<String>, gridsize: Point<i32>) -> i32 {
  let mut robots = Vec::new();
  let re = Regex::new(r"(-?\d+)").unwrap();
  for line in input {
    let captures: Vec<i32> = re
      .find_iter(line)
      .map(|m| m.as_str().parse::<i32>().unwrap())
      .collect();
    robots.push(Robot {
      pos: Point::new(captures[0], captures[1]),
      vel: Point::new(captures[2], captures[3]),
    });
  }

  let file = File::create("output.txt").expect("Filed to create file!");
  let mut file = LineWriter::new(file);
  let mut turn = 1;
  loop {
    for robot in robots.iter_mut() {
      robot.pos = Point::new(
        (robot.pos.x + robot.vel.x).rem_euclid(gridsize.x),
        (robot.pos.y + robot.vel.y).rem_euclid(gridsize.y),
      );
    }
    if turn == 7138 {
      print_grid(&robots, gridsize, &mut file);
      //println!("^^^Turn^^^: {}", turn);
      //file.write_all(format!("^^^{}^^^\n", turn).as_bytes()).expect("Failed to write to file!");
      break;
    }
    // I noticed after scanning the output that starting at turn 31 every 103 turns there was a
    // weird horizontal pattern. Starting at turn 68 every 101 turns there was a weird vertical
    // pattern. I narrowed it down by printing these to a file which made it much easier to scan.
    // I am too lazy to mathematically derive the number but you could probably find it with this
    // information.
    /*
    if (turn-31)%103 == 0 || (turn - 68)%101 == 0 {
      print_grid(&robots, gridsize, &mut file);
      file.write_all(format!("^^^{}^^^\n", turn).as_bytes()).expect("Failed to write to file!");
    }
    */
    turn += 1;
  }
  turn
}

#[cfg(test)]
mod day14_tests {
  const TEST_GRIDSIZE: Point<i32> = Point { x: 11, y: 7 };
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      12,
      solve_part1(
        &inp::parse_file("test_inputs/day14_test.txt"),
        TEST_GRIDSIZE
      )
    );
  }
}
