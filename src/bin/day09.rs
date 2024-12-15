use advent_of_code_2024::inp;
use std::vec::Vec;

const RADIX: u32 = 10;

fn main() {
  let vec = inp::parse_file("inputs/day09.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i64 {
  let input = input[0]
    .chars()
    .map(|x| x.to_digit(RADIX).unwrap() as i32)
    .collect::<Vec<i32>>();
  let mut values = Vec::new();
  let mut spaces = Vec::new();
  let mut flipflop = true;
  for val in input {
    if flipflop {
      values.push(val);
    } else {
      spaces.push(val);
    }
    flipflop = !flipflop;
  }

  let mut checksum: i64 = 0;
  let mut position = 0;
  let mut first_it = 0;
  let mut last_it = values.len() - 1;
  let mut first_val = values[first_it];
  let mut last_val = values[last_it];
  let mut space_it = 0;
  let mut space_val = spaces[space_it];
  let mut is_file = true;
  while first_it < last_it {
    if is_file {
      checksum += position as i64 * first_it as i64;
      first_val -= 1;

      if first_val <= 0 {
        is_file = false;
        while first_val <= 0 {
          first_it += 1;
          first_val = values[first_it];
        }
      }
    } else if space_val > 0 {
      checksum += position as i64 * last_it as i64;
      last_val -= 1;
      space_val -= 1;
      if last_val <= 0 {
        while last_val <= 0 {
          last_it -= 1;
          last_val = values[last_it];
        }
      }
    } else {
      is_file = true;
      space_it += 1;
      space_val = spaces[space_it];
      position -= 1;
    }
    position += 1;
  }
  while last_val > 0 {
    checksum += position as i64 * last_it as i64;
    position += 1;
    last_val -= 1;
  }
  checksum as i64
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Thing {
  size: i64,
  id_num: i64,
  is_space: bool,
}

fn printfiles(files: &Vec<Thing>) {
  for thing in files {
    let mut i = thing.size;
    while i > 0 {
      if thing.is_space {
        print!(".");
      } else {
        print!("{},", thing.id_num);
      }
      i -= 1;
    }
  }
  println!();
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i64 {
  let input = input[0]
    .chars()
    .map(|x| x.to_digit(RADIX).unwrap() as i64)
    .collect::<Vec<i64>>();
  let mut drive = Vec::new();
  let mut flipflop = true;

  for (i, &val) in input.iter().enumerate() {
    if flipflop {
      drive.push(Thing {
        size: val,
        id_num: (i as i64) / 2,
        is_space: false,
      });
    } else {
      drive.push(Thing {
        size: val,
        id_num: -1,
        is_space: true,
      });
    }
    flipflop = !flipflop;
  }
  //printfiles(&drive);

  // Compression
  let reversed_files = drive.clone();
  let reversed_files = reversed_files.iter().filter(|x| !x.is_space).rev();
  for move_candidate in reversed_files {
    for (i, thing) in drive.iter_mut().enumerate() {
      if thing == move_candidate {
        break;
      }
      if !thing.is_space {
        continue;
      }
      if thing.size >= move_candidate.size {
        thing.size = thing.size - move_candidate.size;
        let file_it = drive
          .iter()
          .position(|&x| x.id_num == move_candidate.id_num)
          .unwrap();
        drive.insert(
          file_it,
          Thing {
            size: move_candidate.size,
            id_num: -1,
            is_space: true,
          },
        );
        drive.remove(file_it + 1);
        drive.insert(i, move_candidate.clone());
        break;
      }
    }
  }

  let mut checksum: i64 = 0;
  let mut position = 0;
  for thing in &drive {
    if thing.is_space {
      position += thing.size;
      continue;
    }
    let mut size = thing.size;
    while size > 0 {
      checksum += position * thing.id_num;
      size -= 1;
      position += 1;
    }
  }
  //printfiles(&drive);
  checksum as i64
}

#[cfg(test)]
mod day09_tests {
  use super::*;
  #[test]
  fn test() {
    /*
    assert_eq!(
      1928,
      solve_part1(&inp::parse_file("test_inputs/day09_test.txt"))
    );
    */
    assert_eq!(
      2858,
      solve_part2(&inp::parse_file("test_inputs/day09_test.txt"))
    );
  }
}
