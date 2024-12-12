use std::ops::{Sub,Add};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
  pub x: i32,
  pub y: i32
}

impl Point {
  pub fn new(x: i32, y: i32) -> Point {
    Point{ x, y}
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
