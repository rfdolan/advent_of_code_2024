use advent_of_code_2024::{inp, point::Point};
use std::collections::HashSet;
use std::vec::Vec;

const UP: Point<i32> = Point::<i32> { x: 0, y: -1 };
const RIGHT: Point<i32> = Point::<i32> { x: 1, y: 0 };
const DOWN: Point<i32> = Point::<i32> { x: 0, y: 1 };
const LEFT: Point<i32> = Point::<i32> { x: -1, y: 0 };

#[derive(Eq, PartialEq, Hash, Debug)]
struct Box {
  left_side: Point<i32>,
  right_side: Point<i32>,
}

impl Box {
  fn new(side1: Point<i32>, side2: Point<i32>) -> Box {
    // leftside.x must be less than rightside.x
    if side1.x > side2.x {
      Box {
        left_side: side2,
        right_side: side1,
      }
    } else {
      Box {
        left_side: side1,
        right_side: side2,
      }
    }
  }
  fn contains_point(&self, point: &Point<i32>) -> bool {
    point == &self.left_side || point == &self.right_side
  }
  fn overlaps(&self, other: &Self) -> bool {
    self.contains_point(&other.left_side)
      || self.contains_point(&other.right_side)
  }
  fn move_box(&self, direction: &Point<i32>) -> Box {
    Box {
      left_side: self.left_side + *direction,
      right_side: self.right_side + *direction,
    }
  }
}

fn main() {
  let vec = inp::parse_file("inputs/day15.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn move_object(
  object: &Point<i32>,
  direction: &Point<i32>,
  walls: &HashSet<Point<i32>>,
  boxes: &mut HashSet<Point<i32>>,
) -> Point<i32> {
  let next_pos = *object + *direction;
  // Moving into a wall, can't go further.
  if walls.contains(&next_pos) {
    return *object;
  }
  // Not a wall or a box, we're free to move!
  if !boxes.contains(&next_pos) {
    return next_pos;
  }
  let potential_next = move_object(&next_pos, direction, walls, boxes);
  // Hit a wall somewhere down the line, so we can't move
  if potential_next == next_pos {
    return *object;
  }
  boxes.insert(potential_next);
  return next_pos;
}

fn move_robot(
  robot: &Point<i32>,
  walls: &HashSet<Point<i32>>,
  boxes: &mut HashSet<Point<i32>>,
  direction: &Point<i32>,
) -> Point<i32> {
  let new_pos = move_object(robot, direction, walls, boxes);
  // We moved successfully, so remove the box and put the robot there.
  if new_pos != *robot {
    boxes.remove(&new_pos);
    return new_pos;
  }
  *robot
}

fn parse_input(
  input: &Vec<String>,
) -> (
  HashSet<Point<i32>>,
  HashSet<Point<i32>>,
  Point<i32>,
  Vec<char>,
) {
  let mut walls = HashSet::new();
  let mut boxes = HashSet::new();
  let mut robot = Point::<i32>::new(0, 0);
  let mut y: i32 = 0;
  for line in input.into_iter() {
    if line == "" {
      break;
    }
    let mut x = 0;
    for c in line.chars() {
      match c {
        '#' => {
          walls.insert(Point::new(x, y));
        }
        'O' => {
          boxes.insert(Point::new(x, y));
        }
        '@' => {
          robot = Point::new(x, y);
        }
        _ => {}
      }
      x += 1;
    }
    y += 1;
  }
  let moves = input[y as usize..]
    .iter()
    .map(|x| x.chars())
    .flatten()
    .collect::<Vec<char>>();
  (walls, boxes, robot, moves)
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let (walls, mut boxes, mut robot, moves) = parse_input(input);

  for m in moves {
    match m {
      '^' => robot = move_robot(&robot, &walls, &mut boxes, &UP),
      '>' => robot = move_robot(&robot, &walls, &mut boxes, &RIGHT),
      'v' => robot = move_robot(&robot, &walls, &mut boxes, &DOWN),
      '<' => robot = move_robot(&robot, &walls, &mut boxes, &LEFT),
      _ => println!("Invalid move"),
    }
  }
  boxes.iter().fold(0, |acc, b| acc + (100 * b.y) + b.x)
}

fn can_move(
  box_to_move: &Box,
  direction: &Point<i32>,
  walls: &HashSet<Point<i32>>,
  boxes: &HashSet<Box>,
) -> bool {
  // If we're trying to move a box that isn't there, we're able to move
  if !boxes.contains(box_to_move) {
    return true;
  }
  let next_box = box_to_move.move_box(direction);
  // Moving into a wall, can't go further.
  if walls.contains(&next_box.left_side) || walls.contains(&next_box.right_side)
  {
    return false;
  }
  // Not a wall or a box, we're free to move!
  if boxes
    .iter()
    .all(|b| *b == *box_to_move || !b.overlaps(&next_box))
  {
    return true;
  }
  if direction.x.abs() > 0 {
    // Need to add direction again because the box is 2 wide and we're checking that box
    return can_move(&next_box.move_box(direction), direction, walls, boxes);
  }
  // Moves in the Y direction.
  can_move(&next_box, direction, walls, boxes)
    && can_move(&next_box.move_box(&RIGHT), direction, walls, boxes)
    && can_move(&next_box.move_box(&LEFT), direction, walls, boxes)
}

fn move_box(
  box_to_move: &Box,
  direction: &Point<i32>,
  walls: &HashSet<Point<i32>>,
  boxes: &mut HashSet<Box>,
) {
  if !boxes.contains(box_to_move) {
    return;
  }
  let next_box = box_to_move.move_box(direction);
  if direction.x.abs() > 0 {
    // Need to add direction again because the box is 2 wide and we're checking that box
    move_box(&next_box.move_box(direction), direction, walls, boxes);
  } else {
    // There's 3 possible boxes: directly above, upleft, and upright.
    move_box(&next_box, direction, walls, boxes);
    move_box(&next_box.move_box(&RIGHT), direction, walls, boxes);
    move_box(&next_box.move_box(&LEFT), direction, walls, boxes);
  }
  boxes.remove(box_to_move);
  boxes.insert(next_box);
}

fn move_robot2(
  robot: &Point<i32>,
  walls: &HashSet<Point<i32>>,
  boxes: &mut HashSet<Box>,
  direction: &Point<i32>,
) -> Point<i32> {
  // Walking into the wall won't get you anywhere bub
  if walls.contains(&(*robot + *direction)) {
    return *robot;
  }
  if direction.x.abs() > 0 {
    let bordering_box = Box::new(*robot + *direction, *robot + *direction * 2);
    // No box to our side
    if !boxes.contains(&bordering_box) {
      return *robot + *direction;
    }
    // Box to our side was movable
    if can_move(&bordering_box, direction, walls, boxes) {
      move_box(&bordering_box, direction, walls, boxes);
      return *robot + *direction;
    }
    return *robot;
  }
  let upleft =
    Box::new(*robot + *direction, *robot + Point::new(-1, direction.y));
  let upright =
    Box::new(*robot + *direction, *robot + Point::new(1, direction.y));
  // No box in the way, move freely
  if !boxes.contains(&upleft) && !boxes.contains(&upright) {
    return *robot + *direction;
  }
  // Check if the each possible box exists and can be moved
  if boxes.contains(&upleft) && can_move(&upleft, direction, walls, boxes) {
    move_box(&upleft, direction, walls, boxes);
    return *robot + *direction;
  }
  if boxes.contains(&upright) && can_move(&upright, direction, walls, boxes) {
    move_box(&upright, direction, walls, boxes);
    return *robot + *direction;
  }
  *robot
}

fn _printboard(
  walls: &HashSet<Point<i32>>,
  boxes: &HashSet<Box>,
  robot: &Point<i32>,
  width: i32,
  height: i32,
) {
  for y in 0..height {
    for x in 0..width {
      let p = Point::new(x, y);
      if walls.contains(&p) {
        print!("#");
      } else if robot == &p {
        print!("@");
      } else if boxes.iter().any(|b| b.left_side == p) {
        print!("[");
      } else if boxes.iter().any(|b| b.right_side == p) {
        print!("]");
      } else {
        print!(".");
      }
    }
    println!();
  }
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let mut walls = HashSet::new();
  let mut robot = Point::<i32>::new(0, 0);
  let mut boxes: HashSet<Box> = HashSet::new();
  let mut y: i32 = 0;
  for line in input.into_iter() {
    if line == "" {
      break;
    }
    let mut x = 0;
    for c in line.chars() {
      match c {
        '#' => {
          walls.insert(Point::new(x * 2, y));
          walls.insert(Point::new((x * 2) + 1, y));
        }
        'O' => {
          boxes.insert(Box {
            left_side: Point::new(x * 2, y),
            right_side: Point::new((x * 2) + 1, y),
          });
        }
        '@' => {
          robot = Point::new(x * 2, y);
        }
        _ => {}
      }
      x += 1;
    }
    y += 1;
  }
  //let size = ((input[0].len() * 2) as i32, y);
  let moves = input[y as usize..]
    .iter()
    .map(|x| x.chars())
    .flatten()
    .collect::<Vec<char>>();

  for m in moves {
    //println!("Move: {}", m);

    match m {
      '^' => robot = move_robot2(&robot, &walls, &mut boxes, &UP),
      '>' => robot = move_robot2(&robot, &walls, &mut boxes, &RIGHT),
      'v' => robot = move_robot2(&robot, &walls, &mut boxes, &DOWN),
      '<' => robot = move_robot2(&robot, &walls, &mut boxes, &LEFT),
      _ => println!("Invalid move"),
    }
    //printboard(&walls, &boxes, &robot, size.0, size.1);
  }
  boxes
    .iter()
    .fold(0, |acc, b| acc + (100 * b.left_side.y) + b.left_side.x)
}

#[cfg(test)]
mod day15_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(
      2028,
      solve_part1(&inp::parse_file("test_inputs/day15_test1.txt"))
    );
    assert_eq!(
      10092,
      solve_part1(&inp::parse_file("test_inputs/day15_test2.txt"))
    );
    assert_eq!(
      9021,
      solve_part2(&inp::parse_file("test_inputs/day15_test2.txt"))
    );
  }
}
