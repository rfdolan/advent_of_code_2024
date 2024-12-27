use std::ops::{Add, Mul, Sub};


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point<T> {
  pub x: T,
  pub y: T,
}

impl<T> Point<T> {
  pub fn new(x: T, y: T) -> Point<T> {
    Point { x, y }
  }

  pub fn zero() -> Point<T>
  where
    T: From<i32>,
  {
    Point {
      x: T::from(0),
      y: T::from(0),
    }
  }

  pub fn new_from_tuple(vals: (T, T)) -> Point<T> {
    Point {
      x: vals.0,
      y: vals.1,
    }
  }

  pub fn neighbors(&self) -> Vec<Point<T>>
  where
    T: Add<T, Output = T> + Sub<T, Output = T> + Copy + From<i32>,
  {
    vec![
      Point::new(self.x + T::from(1), self.y),
      Point::new(self.x - T::from(1), self.y),
      Point::new(self.x, self.y + T::from(1)),
      Point::new(self.x, self.y - T::from(1)),
    ]
  }
  pub fn neighbors_diagonal(&self) -> Vec<Point<T>>
  where
    T: Add<T, Output = T> + Sub<T, Output = T> + Copy + From<i32>,
  {
    vec![
      Point::new(self.x + T::from(1), self.y),
      Point::new(self.x + T::from(1), self.y + T::from(1)),
      Point::new(self.x + T::from(1), self.y - T::from(1)),
      Point::new(self.x - T::from(1), self.y),
      Point::new(self.x - T::from(1), self.y + T::from(1)),
      Point::new(self.x - T::from(1), self.y - T::from(1)),
      Point::new(self.x, self.y + T::from(1)),
      Point::new(self.x, self.y - T::from(1)),
    ]
  }
  pub fn manhattan_dist(self, other: Self) -> i32 
  where 
  T: Add<T, Output = T> + Sub<T, Output = T>,
  i32: From<T>
  {
    i32::from(self.x - other.x).abs() + i32::from(self.y - other.y).abs()
  }

}

impl<T> Add for Point<T>
where
  T: Add<T, Output = T> + Copy,
{
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl<T> Sub for Point<T>
where
  T: Sub<T, Output = T> + Copy,
{
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl<T> Mul<T> for Point<T>
where
  T: Mul<T, Output = T> + Copy,
{
  type Output = Self;
  fn mul(self, m: T) -> Self {
    Self {
      x: self.x * m,
      y: self.y * m,
    }
  }
}

pub const CARDINAL_DIRS: [Point<i32>; 4] = [Point{x: 1, y:0}, Point{x: -1, y:0}, Point{x:0, y:1}, Point{x:0, y:-1} ];