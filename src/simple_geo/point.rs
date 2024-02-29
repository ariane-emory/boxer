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

  pub fn invert(&self) -> Self {
    Self::new(self.col, self.line)
  }

  pub fn offset(&self, line_offset: isize, col_offset: isize) -> Self {
    // if adding the offsets to the existing line/column would over/underflow,
    // something has gone wrong, panic.

    if line_offset < 0 && self.line < (-line_offset) as usize {
      panic!("Line underflow");
    }

    if col_offset < 0 && self.col < (-col_offset) as usize {
      panic!("Column underflow");
    }

    if line_offset > 0 && self.line > (usize::MAX as isize - line_offset) as usize {
      panic!("Line overflow");
    }

    if col_offset > 0 && self.col > (usize::MAX as isize - col_offset) as usize {
      panic!("Column overflow");
    }

    Self::new(
      (self.line as isize + line_offset) as usize,
      (self.col as isize + col_offset) as usize,
    )
  }
}
