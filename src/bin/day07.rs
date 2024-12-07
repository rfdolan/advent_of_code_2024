use std::vec::Vec;

fn main(){
  let vec = inp::parse_file("inputs/day07.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn test_recursive(target: i64, nums: &Vec<i64>, useor: bool) -> bool {
  if nums.len() == 1 {
    return nums[0] == target;
  }
  if nums[nums.len() - 1] >= target {
    return false;
  }
  let num1 = nums[nums.len() - 1];
  let num2 = nums[nums.len() - 2];
  let addresult = num1 + num2;
  let mulresult = num1 * num2;

  let mut addvec = nums[..nums.len() - 2].to_vec();
  addvec.push(addresult);
  let mut mulvec = nums[..nums.len() - 2].to_vec();
  mulvec.push(mulresult);
  if useor {
    let orresult = (num1.to_string() + &num2.to_string()).parse::<i64>().unwrap();
    let mut orvec = nums[..nums.len() - 2].to_vec();
    orvec.push(orresult);
    return test_recursive(target, &addvec, true) || 
           test_recursive(target, &mulvec, true) || 
           test_recursive(target, &orvec, true);
  }

  test_recursive(target, &addvec, false) || 
  test_recursive(target, &mulvec, false)
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i64 {
  let mut total_calibartion_result = 0;
  for line in input {
    let split = line.split(": ").collect::<Vec<_>>();
    let test_value = split[0].parse::<i64>().unwrap();
    let mut numbers = split[1].split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    numbers.reverse();
    if test_recursive(test_value, &numbers, false) {
      total_calibartion_result += test_value;
    }
  }
  total_calibartion_result
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i64 {
  let mut total_calibartion_result = 0;
  for line in input {
    let split = line.split(": ").collect::<Vec<_>>();
    let test_value = split[0].parse::<i64>().unwrap();
    let mut numbers = split[1].split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    numbers.reverse();
    if test_recursive(test_value, &numbers, true) {
      total_calibartion_result += test_value;
    }
  }
  total_calibartion_result
}

#[cfg(test)]
mod day07_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(3749, solve_part1(&inp::parse_file("test_inputs/day07_test.txt")));
    assert_eq!(11387, solve_part2(&inp::parse_file("test_inputs/day07_test.txt")));
  }
}