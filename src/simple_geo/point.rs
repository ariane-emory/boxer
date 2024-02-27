use crate::simple_geo::*;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
  pub line: usize,
  pub col: usize,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({:2}:{:2})", self.col, self.line)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Positional for Point {
  fn top_left(&self) -> Point {
    *self
  }

  fn bottom_right(&self) -> Point {
    *self
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Point {
  pub fn new(col: usize, line: usize) -> Point {
    Point { col, line }
  }

  pub fn is_vertically_aligned_with(&self, other: &Self) -> bool {
    self.col == other.col
  }

  pub fn is_horizontally_aligned_with(&self, other: &Self) -> bool {
    self.line == other.line
  }

  pub fn distance(&self, other: &Self) -> usize {
    // calculates the 'Manhattan distance'.
    (self.col as isize - other.col as isize).abs() as usize
      + (self.line as isize - other.line as isize).abs() as usize
  }
}
