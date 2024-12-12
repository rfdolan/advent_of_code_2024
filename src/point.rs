use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Point {
    Point { x, y }
  }
  pub fn neighbors(&self) -> Vec<Point> {
    vec![
      Point::new(self.x + 1, self.y),
      Point::new(self.x - 1, self.y),
      Point::new(self.x, self.y + 1),
      Point::new(self.x, self.y - 1),
    ]
  }
  pub fn neighbors_diagonal(&self) -> Vec<Point> {
    vec![
      Point::new(self.x + 1, self.y),
      Point::new(self.x + 1, self.y + 1),
      Point::new(self.x + 1, self.y - 1),
      Point::new(self.x - 1, self.y),
      Point::new(self.x - 1, self.y + 1),
      Point::new(self.x - 1, self.y - 1),
      Point::new(self.x, self.y + 1),
      Point::new(self.x, self.y - 1),
    ]
  }
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
