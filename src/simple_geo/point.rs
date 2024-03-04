use crate::simple_geo::*;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
  pub line: usize,
  pub col: usize,
}

////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({:2}:{:2})", self.line, self.col)
  }
}

////////////////////////////////////////////////////////////////////////////////
impl Positional for Point {
  fn top_left(&self) -> Self {
    *self
  }

  fn bottom_right(&self) -> Self {
    *self
  }
}

////////////////////////////////////////////////////////////////////////////////
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
impl Offsetable for Point {
  fn flip(&self) -> Self {
    Self::new(self.col, self.line)
  }

  fn offset_by(&self, line_offset: isize, col_offset: isize) -> Self {
    let new_line = if line_offset < 0 {
      self.line.checked_sub((-line_offset) as usize)
    }
    else {
      self.line.checked_add(line_offset as usize)
    };

    let new_col = if col_offset < 0 {
      self.col.checked_sub((-col_offset) as usize)
    }
    else {
      self.col.checked_add(col_offset as usize)
    };

    // Construct new Point or panic on error
    match (new_line, new_col) {
      (Some(line), Some(col)) => Self::new(line, col),
      _ => panic!("Offset results in underflow or overflow"),
    }
  }
}
