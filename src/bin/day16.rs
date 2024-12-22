use advent_of_code_2024::{inp, point::Point};
use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;

const NORTH: Point<i32> = Point::<i32> { x: 0, y: -1 };
const EAST: Point<i32> = Point::<i32> { x: 1, y: 0 };
const SOUTH: Point<i32> = Point::<i32> { x: 0, y: 1 };
const WEST: Point<i32> = Point::<i32> { x: -1, y: 0 };

fn main() {
  let vec = inp::parse_file("inputs/day16.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn turn(dir: &Point<i32>, turn_dir: char) -> Point<i32> {
  match *dir {
    NORTH => match turn_dir {
      'L' => WEST,
      'R' => EAST,
      _ => NORTH,
    },
    EAST => match turn_dir {
      'L' => NORTH,
      'R' => SOUTH,
      _ => EAST,
    },
    SOUTH => match turn_dir {
      'L' => EAST,
      'R' => WEST,
      _ => SOUTH,
    },
    WEST => match turn_dir {
      'L' => SOUTH,
      'R' => NORTH,
      _ => WEST,
    },
    _ => Point::zero(),
  }
}

fn printboard(
  walls: &HashSet<Point<i32>>,
  size: (usize, usize),
  dists: &HashMap<Point<i32>, (i32, Point<i32>)>,
) {
  for y in 0..size.1 {
    for x in 0..size.0 {
      let p = Point::new(x as i32, y as i32);
      if walls.contains(&p) {
        print!("\t#");
      } else {
        match dists.get(&p) {
          Some(d) => {
            if d.0 > 100000 {
              print!("\tX");
            } else {
              print!("\t{}", d.0);
            }
            /*
            match d.1 {
              NORTH => print!("^"),
              EAST => print!(">"),
              SOUTH => print!("v"),
              WEST => print!("<"),
              _ => print!("?"),
            }
            */
          }
          None => {
            print!("\t");
          }
        }
      }
    }
    println!();
  }
}

fn dijkstras(
  start: &Point<i32>,
  walls: &HashSet<Point<i32>>,
  size: (usize, usize),
) -> HashMap<Point<i32>, (i32, Point<i32>)> {
  let mut distances = HashMap::new();
  let mut visited = HashSet::new();
  let mut to_visit = VecDeque::new();
  // Initialize all distances to infinite
  for y in 0..size.1 {
    for x in 0..size.0 {
      distances.insert(Point::new(x as i32, y as i32), (std::i32::MAX, EAST));
    }
  }

  distances.insert(*start, (0, EAST));
  to_visit.push_back(*start);
  while let Some(current) = to_visit.pop_front() {
    visited.insert(current);
    let current_dist = distances[&current];
    let straight = current + current_dist.1;
    if !walls.contains(&straight) {
      let new_distance = current_dist.0 + 1;
      match distances.get(&straight) {
        Some(&d) => {
          if new_distance < d.0 {
            distances.insert(straight, (new_distance, current_dist.1));
            to_visit.push_back(straight);
          }
        }
        None => {
          distances.insert(straight, (new_distance, current_dist.1));
          to_visit.push_back(straight);
        }
      }
    } else {
    }
    let left_turn = current + turn(&current_dist.1, 'L');
    if !walls.contains(&left_turn) {
      let new_distance = current_dist.0 + 1000 + 1;
      match distances.get(&left_turn) {
        Some(&d) => {
          if new_distance < d.0 {
            distances
              .insert(left_turn, (new_distance, turn(&current_dist.1, 'L')));
            to_visit.push_back(left_turn);
          }
        }
        None => {
          distances
            .insert(left_turn, (new_distance, turn(&current_dist.1, 'L')));
          to_visit.push_back(left_turn);
        }
      }
    }
    let right_turn = current + turn(&current_dist.1, 'R');
    if !walls.contains(&right_turn) {
      let new_distance = current_dist.0 + 1000 + 1;
      match distances.get(&right_turn) {
        Some(&d) => {
          if new_distance < d.0 {
            distances
              .insert(right_turn, (new_distance, turn(&current_dist.1, 'R')));
            to_visit.push_back(right_turn);
          }
        }
        None => {
          distances
            .insert(right_turn, (new_distance, turn(&current_dist.1, 'R')));
          to_visit.push_back(right_turn);
        }
      }
    }
  }
  distances
}

fn get_input(
  input: &Vec<String>,
) -> (HashSet<Point<i32>>, Point<i32>, Point<i32>, (usize, usize)) {
  let mut walls = HashSet::new();
  let mut reindeer_pos: Point<i32> = Point::zero();
  let mut end: Point<i32> = Point::zero();
  let size = (input[0].len(), input.len());
  for (y, line) in input.iter().enumerate() {
    for (x, c) in line.chars().enumerate() {
      match c {
        '#' => {
          walls.insert(Point::new(x as i32, y as i32));
        }
        'S' => {
          reindeer_pos = Point::new(x as i32, y as i32);
        }
        'E' => {
          end = Point::new(x as i32, y as i32);
        }
        _ => {}
      }
    }
  }
  (walls, reindeer_pos, end, size)
}

fn back_search_num_nodes(
  minimum_spanning_tree: &HashMap<Point<i32>, (i32, Point<i32>)>,
  start: &Point<i32>,
  walls: &HashSet<Point<i32>>,
  size: (usize, usize),
) -> i32 {
  let mut visited = HashSet::new();
  let mut to_visit = Vec::new();
  visited.insert(*start);
  to_visit.push(*start);
  while let Some(curr) = to_visit.pop() {
    for neighbor in curr.neighbors() {
      let (curr_cost, _curr_dir) = minimum_spanning_tree.get(&curr).unwrap();
      match minimum_spanning_tree.get(&neighbor) {
        Some(&(next_cost, next_dir)) => {
          if !visited.contains(&neighbor) && next_cost < *curr_cost {
            //println!("{:?}", neighbor);
            visited.insert(neighbor);
            to_visit.push(neighbor);
          }
        }
        None => {}
      }
    }
  }
  printseats(&visited, walls, size);

  visited.len() as i32
}

fn back_search_rec(
  minimum_spanning_tree: &HashMap<Point<i32>, (i32, Point<i32>)>,
  curr: &Point<i32>,
  target: &Point<i32>,
  start: &Point<i32>,
) -> HashSet<Point<i32>> {
  if curr == target {
    let mut v = HashSet::new();
    v.insert(*curr);
    return v;
  }
  match minimum_spanning_tree.get(curr) {
    Some((cost, dir)) => {
      let mut visited = HashSet::new();
      let prev = *curr - *dir;
      if minimum_spanning_tree[&prev].0 < *cost {
        let res = back_search_rec(minimum_spanning_tree, &prev, target, start);
        if res.len() > 0 {
          visited.extend(res);
          visited.insert(*curr);
        }
      }
      let prev_to_left = *curr - turn(dir, 'L');
      if let Some((left_cost, left_dir)) =
        minimum_spanning_tree.get(&prev_to_left)
      {
        if *left_cost <= *cost + 999
          && *left_dir == turn(dir, 'L')
          && *curr != *start
        {
          let res = back_search_rec(
            minimum_spanning_tree,
            &prev_to_left,
            target,
            start,
          );
          if res.len() > 0 {
            visited.extend(res);
            visited.insert(*curr);
          }
        }
      }
      let prev_to_right = *curr - turn(dir, 'R');
      if let Some((right_cost, right_dir)) =
        minimum_spanning_tree.get(&prev_to_right)
      {
        if *right_cost <= *cost + 999
          && *right_dir == turn(dir, 'R')
          && *curr != *start
        {
          let res = back_search_rec(
            minimum_spanning_tree,
            &prev_to_right,
            target,
            start,
          );
          if res.len() > 0 {
            visited.extend(res);
            visited.insert(*curr);
          }
        }
      }
      visited
    }
    None => HashSet::new(),
  }
}

fn dfs(
  start: &Point<i32>,
  end: &Point<i32>,
  prev: &Point<i32>,
  walls: &HashSet<Point<i32>>,
  d: &HashMap<Point<i32>, (i32, Point<i32>)>,
) -> HashSet<Point<i32>> {
  let mut visited = HashSet::new();
  if walls.contains(start) {
    return visited;
  }
  //println!("Checking{:?}", start);

  if *start == *end {
    if d[prev].0 > d[end].0 {
      return visited;
    }
    println!("Found end!");
    visited.insert(*start);
    return visited;
  }
  let (curr_dist, curr_dir) = d[start];
  for neighbor in start.neighbors() {
    if neighbor == *prev {
      continue;
    }
    match d.get(&neighbor) {
      Some(&(dist, dir)) => {
        // Only add things that are strictly larger, we don't want to visit anthing that would take us away.
        if dist > curr_dist {
          let visited_on_path = dfs(&neighbor, end, start, walls, d);
          if visited_on_path.len() > 0 {
            //println!("FOUND: {:?}", start);
            visited.extend(visited_on_path);
            visited.insert(*start);
          }
        } else if curr_dir.x.abs() != dir.x.abs()  && dist+999 <= curr_dist {
              let visited_on_path = dfs(&neighbor, end,  start,walls, d);
              if visited_on_path.len() > 0 {
           //     println!("FOUND: {:?}", start);
                visited.extend(visited_on_path);
                visited.insert(*start);
          }
        } else {
          //println!("Didn't work for {:?}, didn't fulfill req", start);
        }
      },
      None => ()
    }
  }
  visited
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let (walls, reindeer_pos, end, size) = get_input(input);

  let d = dijkstras(&reindeer_pos, &walls, size);
  //printboard(&walls, size, &d);
  d[&end].0 
}

fn printseats(
  seats: &HashSet<Point<i32>>,
  walls: &HashSet<Point<i32>>,
  size: (usize, usize),
) {
  for y in 0..size.1 {
    for x in 0..size.0 {
      let p = Point::new(x as i32, y as i32);
      if walls.contains(&p) {
        print!("#");
        continue;
      }
      if seats.contains(&p) {
        print!("O");
        continue;
      }
      print!(".");
    }
    println!();
  }
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let (walls, reindeer_pos, end, size) = get_input(input);

  // Heurestic? Whatever for?
  let d = dijkstras(&reindeer_pos, &walls, size);

  printboard(&walls, size, &d);
  //let end_dist = d[&end].0;
  // Search backwards through our minimum tree
  let res = dfs( &reindeer_pos, &end, &Point::new(-1,-1), &walls, &d);
  //let res = back_search_rec(&d, &end, &reindeer_pos, &end);
  printseats(&res, &walls, size);
  res.len() as i32
}

#[cfg(test)]
mod day16_tests {
  use super::*;
  #[test]
  fn test() {
    /*
    assert_eq!(
      7036,
      solve_part1(&inp::parse_file("test_inputs/day16_test1.txt"))
    );
    assert_eq!(
      11048,
      solve_part1(&inp::parse_file("test_inputs/day16_test2.txt"))
    );
    */
    assert_eq!(
      45,
      solve_part2(&inp::parse_file("test_inputs/day16_test1.txt"))
    );
    assert_eq!(
      64,
      solve_part2(&inp::parse_file("test_inputs/day16_test2.txt"))
    );
    assert_eq!(
      53,
      solve_part2(&inp::parse_file("test_inputs/day16_test3.txt"))
    );
  }
}
