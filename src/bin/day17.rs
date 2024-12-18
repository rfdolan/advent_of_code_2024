use advent_of_code_2024::inp;
use regex::Regex;
use std::collections::HashMap;
use std::{ops::BitXor, vec::Vec};

fn main() {
  let vec = inp::parse_file("inputs/day17.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

// instruction pointer starts at 0 and increases by 2 each call (unless jump)

// literal operand is the number iteslf
// combo operand is the following
// 0-3 is the number
// 4 = regA
// 5 = regB
// 6 = regC

// 0 adv (A / 2^combo-op) write to A
// 1 bxl (bitwise xor of B and literal op)
// 2 bst combo operand % 8 write to B
// 3 jnz jump if A is not zero to value of literal op
// 4 bxc bitwise XOR of B and C, storing it in B
// 5 out combo % 8 and outputs it (?)
// 6 bdv adv but stored in B
// 7 cdv adv but stored in C

#[derive(Clone)]
struct Computer {
  rga: u64,
  rgb: u64,
  rgc: u64,
  program: Vec<u64>,
  instr_ptr: u64,
}

impl Computer {
  fn reset(&mut self) {
    self.rga = 0;
    self.rgb = 0;
    self.rgc = 0;
    self.instr_ptr = 0;
  }
  fn do_next(&mut self) -> String {
    if self.instr_ptr as usize >= self.program.len() {
      return "done!".to_string();
    }
    self.process(
      self.program[self.instr_ptr as usize],
      self.program[self.instr_ptr as usize + 1],
    )
  }
  fn get_combo(&self, operand: u64) -> u64 {
    match operand {
      0..=3 => operand,
      4 => self.rga,
      5 => self.rgb,
      6 => self.rgc,
      _ => {
        panic!("Operand {} is invalid!", operand);
      }
    }
  }
  fn adv(&mut self, operand: u64) {
    self.rga = self.rga / ((2 as u64).pow(self.get_combo(operand) as u32));
  }
  fn bxl(&mut self, operand: u64) {
    self.rgb = self.rgb.bitxor(operand);
  }
  fn bst(&mut self, operand: u64) {
    self.rgb = self.get_combo(operand) % 8;
  }
  fn jnz(&mut self, operand: u64) -> bool {
    if self.rga != 0 {
      self.instr_ptr = operand;
      return true;
    }
    false
  }
  fn bxc(&mut self, _: u64) {
    self.rgb = self.rgb.bitxor(self.rgc);
  }
  fn out(&self, operand: u64) -> String {
    (self.get_combo(operand) % 8).to_string()
  }
  fn bdv(&mut self, operand: u64) {
    self.rgb = self.rga / ((2 as u64).pow(self.get_combo(operand) as u32));
  }
  fn cdv(&mut self, operand: u64) {
    self.rgc = self.rga / ((2 as u64).pow(self.get_combo(operand) as u32));
  }
  fn process(&mut self, command: u64, operand: u64) -> String {
    let mut output = "".to_string();
    match command {
      0 => {
        self.adv(operand);
        self.instr_ptr += 2;
      }
      1 => {
        self.bxl(operand);
        self.instr_ptr += 2;
      }
      2 => {
        self.bst(operand);
        self.instr_ptr += 2;
      }
      3 => {
        if !self.jnz(operand) {
          self.instr_ptr += 2;
        }
      }
      4 => {
        self.bxc(operand);
        self.instr_ptr += 2;
      }
      5 => {
        output = self.out(operand);
        self.instr_ptr += 2;
      }
      6 => {
        self.bdv(operand);
        self.instr_ptr += 2;
      }
      7 => {
        self.cdv(operand);
        self.instr_ptr += 2;
      }
      _ => {}
    }
    output
  }
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> String {
  let re = Regex::new(r"(\d+)").unwrap();
  let rga = re.find(&input[0]).unwrap().as_str().parse::<u64>().unwrap();
  let rgb = re.find(&input[1]).unwrap().as_str().parse::<u64>().unwrap();
  let rgc = re.find(&input[2]).unwrap().as_str().parse::<u64>().unwrap();
  let program = re
    .find_iter(&input[4])
    .map(|m| m.as_str().parse::<u64>().unwrap())
    .collect::<Vec<u64>>();

  let mut computer = Computer {
    rga,
    rgb,
    rgc,
    program,
    instr_ptr: 0,
  };

  let mut output = "".to_string();
  loop {
    let out = computer.do_next();
    if out == "done!" {
      return output[..output.len() - 1].to_string();
    }
    if out.len() > 0 {
      output = output + &out + ",";
    }
  }
}

fn run_program(computer: &mut Computer, a_val: u64) -> (String, u64) {
  computer.reset();
  computer.rga = a_val;
  let mut output = "".to_string();
  loop {
    let out = computer.do_next();
    if out == "done!" {
      let final_output = output[..output.len() - 1].to_string();
      return (final_output, a_val);
    }
    if out.len() > 0 {
      output = output + &out + ",";
    }
  }
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> u64 {
  let re = Regex::new(r"(\d+)").unwrap();
  let rga = re.find(&input[0]).unwrap().as_str().parse::<u64>().unwrap();
  let rgb = re.find(&input[1]).unwrap().as_str().parse::<u64>().unwrap();
  let rgc = re.find(&input[2]).unwrap().as_str().parse::<u64>().unwrap();
  let program = re
    .find_iter(&input[4])
    .map(|m| m.as_str().parse::<u64>().unwrap())
    .collect::<Vec<u64>>();

  let mut program_as_string = "".to_string();
  for &val in &program {
    program_as_string = program_as_string + &val.to_string() + ",";
  }
  program_as_string =
    program_as_string[..program_as_string.len() - 1].to_string();
  let mut computer = Computer {
    rga,
    rgb,
    rgc,
    program: program.clone(),
    instr_ptr: 0,
  };

  // No generalized solver, I analyzed the program and broke it down such that my input was
  // represented by the equation:
  // (((A%8)XOR4)XOR(A>>((A%8)XOR1)))%8
  //
  // I generated the below map and dumped it into a spreadsheet, mapping A values to their output
  let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
  for val in 0..0b111111111 {
    let res =
      ((val % 8 as i32).bitxor(4)).bitxor(val >> ((val % 8).bitxor(1))) % 8;
    match map.get_mut(&res) {
      Some(vec) => {
        vec.push(val);
      }
      None => {
        map.insert(res, vec![val]);
      }
    }
    println!("{val} => \t{}", res);
  }
  for entry in map.iter() {
    print!("{} => ", entry.0);
    for val in entry.1 {
      print!(", {val:b}");
    }
    println!();
  }

  // From there I analyzed it with my eyeballs and, working from the highest order bits, constructed
  // the "a_val" below that
  //  1. Fulfilled the output
  //  2. Was the smallest
  // Each position represents an output from my program, in reverse order.
  //                     0   3   3   0   5   5   1   4   5   1   5   7   1   1   4   2
  let mut a_val =
    0b100_101_010_110_100_100_100_001_011_011_010_110_111_010_111_101;

  // This loop is maintained here but with my input it exits immediately.
  loop {
    let res = run_program(&mut computer, a_val);
    //println!("{} gives us {}", a_val, res.0);
    if res.0 == program_as_string {
      return res.1;
    }
    a_val += 1;
  }
}

#[cfg(test)]
mod day17_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      "4,6,3,5,6,3,5,2,1,0".to_string(),
      solve_part1(&inp::parse_file("test_inputs/day17_test1.txt"))
    );
    assert_eq!(
      117440,
      solve_part2(&inp::parse_file("test_inputs/day17_test2.txt"))
    );
  }
}
