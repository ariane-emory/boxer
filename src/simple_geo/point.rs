use crate::simple_geo::*;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
  pub col: usize,
  pub line: usize,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({:2}:{:2})", self.line, self.col)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Positional for Point {
  fn top_left(&self) -> Self {
    *self
  }

  fn bottom_right(&self) -> Self {
    *self
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Point {
  pub fn new(line: usize, col: usize) -> Self {
    Self { line, col }
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
