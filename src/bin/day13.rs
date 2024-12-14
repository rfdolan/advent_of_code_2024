use advent_of_code_2024::{inp, point::Point};
use std::vec::Vec;
use nalgebra::{Matrix2, Matrix2x1};

const PART2_FUDGE: i64 = 10000000000000;

struct Machine {
  button_a: Point<i64>,
  button_b: Point<i64>,
  target: Point<i64>,
}

fn main() {
  let vec = inp::parse_file("inputs/day13.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn get_min_tokens(machine: &Machine) -> i64 {
  let a = Matrix2::new(machine.button_a.x as f64, machine.button_b.x as f64, machine.button_a.y as f64, machine.button_b.y as f64);
  let b = Matrix2x1::new(machine.target.x as f64, machine.target.y as f64);
  match a.lu().solve(&b) {
    Some(solution) => {
      //println!("{:?}", solution); 
      let adiff = solution[0].round() as i64 as f64 - solution[0];
      let bdiff = solution[1].round() as i64 as f64 - solution[1];
      if adiff.abs() > 0.001 || bdiff.abs() > 0.001 {
        return 0;
      }
      return 3* solution[0].round() as i64 + solution[1].round() as i64;
    },
    None => return 0
  }
}

fn get_machines(input: &Vec<String>) -> Vec<Machine> {
  let mut i = 0;
  let mut machines = Vec::new();
  while i < input.len() {
    let vals = input[i]
      .split(",")
      .map(|x| x.chars().filter(|c| c.is_digit(10)).collect::<String>())
      .map(|x| x.parse::<i64>().unwrap())
      .collect::<Vec<i64>>();
    let button_a = Point::new(vals[0], vals[1]);
    let vals = input[i+1]
      .split(",")
      .map(|x| x.chars().filter(|c| c.is_digit(10)).collect::<String>())
      .map(|x| x.parse::<i64>().unwrap())
      .collect::<Vec<i64>>();
    let button_b = Point::new(vals[0], vals[1]);
    let vals = input[i+2]
      .split(",")
      .map(|x| x.chars().filter(|c| c.is_digit(10)).collect::<String>())
      .map(|x| x.parse::<i64>().unwrap())
      .collect::<Vec<i64>>();
    let target = Point::new(vals[0], vals[1]);
    machines.push(Machine {
      button_a,
      button_b,
      target,
    });
    i += 4;
  }
  machines
}
// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i64 {
  get_machines(input)
    .iter()
    .fold(0, |acc, machine| get_min_tokens(&machine) + acc)
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i64 {
  let mut machines = get_machines(input);
  for machine in machines.iter_mut() {
    machine.target = Point::new(machine.target.x + PART2_FUDGE, machine.target.y + PART2_FUDGE);
  }
  machines.iter()
    .fold(0, |acc, machine| get_min_tokens(&machine) + acc)
}

#[cfg(test)]
mod day13_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      480,
      solve_part1(&inp::parse_file("test_inputs/day13_test.txt"))
    );
  }
}
