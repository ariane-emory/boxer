#![allow(dead_code)]
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct ErrString {
  string: String,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ErrString {
  pub fn new(string: &str) -> ErrString {
    ErrString {
      string: string.to_string(),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Error for ErrString {}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Display for ErrString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error: {}", self.string)
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size {
  pub height: u64,
  pub width: u64,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Size {
  pub fn new(height: u64, width: u64) -> Size {
    Size { height, width }
  }

  pub fn area(&self) -> u64 {
    self.height * self.width
  }

  pub fn is_tall(&self) -> bool {
    self.height > self.width
  }

  pub fn is_wide(&self) -> bool {
    self.width > self.height
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Positional {
  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn left_bound(&self) -> u64;
  fn right_bound(&self) -> u64;
  fn upper_bound(&self) -> u64;
  fn lower_bound(&self) -> u64;
  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn size(&self) -> Size {
    Size::new(
      self.upper_bound() - self.lower_bound() + 1,
      self.right_bound() - self.left_bound() + 1,
    )
  }
  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn is_left_aligned_with(&self, other: &impl Positional) -> bool {
    self.left_bound() == other.left_bound()
  }

  fn is_right_aligned_with(&self, other: &impl Positional) -> bool {
    self.right_bound() == other.right_bound()
  }

  fn is_top_aligned_with(&self, other: &impl Positional) -> bool {
    self.upper_bound() == other.upper_bound()
  }

  fn is_bottom_aligned_with(&self, other: &impl Positional) -> bool {
    self.lower_bound() == other.lower_bound()
  }
  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn is_left_of(&self, other: &impl Positional) -> bool {
    self.right_bound() < other.left_bound()
  }

  fn is_right_of(&self, other: &impl Positional) -> bool {
    !(self.is_left_of(other) || self.is_left_aligned_with(other))
  }

  fn is_above(&self, other: &impl Positional) -> bool {
    self.lower_bound() < other.upper_bound()
  }

  fn is_below(&self, other: &impl Positional) -> bool {
    !(self.is_above(other) || self.is_top_aligned_with(other))
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
  pub line: u64,
  pub col: u64,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Point {
  pub fn new(line: u64, col: u64) -> Point {
    Point { line, col }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Positional for Point {
  fn upper_bound(&self) -> u64 {
    self.line
  }

  fn lower_bound(&self) -> u64 {
    self.line
  }

  fn left_bound(&self) -> u64 {
    self.col
  }

  fn right_bound(&self) -> u64 {
    self.col
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
  pub start: Point,
  pub end: Point,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
type LineResult = std::result::Result<Line, ErrString>;
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Line {
  pub fn new(start: Point, end: Point) -> LineResult {
    if start == end {
      return Err(ErrString::new("Start and end points cannot be the same"));
    }

    if !(start.is_left_aligned_with(&end) || start.is_top_aligned_with(&end)) {
      return Err(ErrString::new("Line must be either horizontal or vertical"));
    }

    let (start, end) = if (start.line < end.line) || (start.col < end.col) {
      (start, end)
    } else {
      (end, start)
    };
    Ok(Line { start, end })
  }

  pub fn is_horizontal(&self) -> bool {
    self.size().is_wide()
  }

  pub fn is_vertical(&self) -> bool {
    self.size().is_tall()
  }

  // pub fn is_parallel_to(&self, other: &Self) -> bool {
  //   (self.is_horizontal() && other.is_horizontal()) || (self.is_vertical() && other.is_vertical())
  // }

  // pub fn could_pair_vertically_with(&self, other: Self) -> bool {
  //   self.start.is_horizontally_aligned_with(&other.start) && self.length() == other.length()
  // }

  // pub fn could_pair_horizontally_with(&self, other: Self) -> bool {
  //   self.start.is_vertically_aligned_with(&other.start) && self.length() == other.length()
  // }

  // pub fn length(&self) -> u64 {
  //   if self.is_horizontal() {
  //     self.end.col - self.start.col
  //   } else {
  //     self.end.line - self.start.line
  //   }
  // }

  // // Whether lines are 'above' or to the left of each other is judged based on their start.
  // pub fn is_above(&self, other: Self) -> bool {
  //   self.start.is_above(&other.start)
  // }

  // pub fn is_below(&self, other: Self) -> bool {
  //   self.start.is_below(&other.start)
  // }

  // pub fn is_left_of(&self, other: Self) -> bool {
  //   self.start.is_left_of(&other.start)
  // }

  // pub fn is_right_of(&self, other: Self) -> bool {
  //   self.start.is_right_of(&other.start)
  // }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for Line {}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Ord for Line {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .start
      .cmp(&other.start)
      .then_with(|| self.end.cmp(&other.end))
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl PartialOrd for Line {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Positional for Line {
  fn upper_bound(&self) -> u64 {
    self.start.upper_bound()
  }

  fn lower_bound(&self) -> u64 {
    self.end.lower_bound()
  }

  fn left_bound(&self) -> u64 {
    self.start.left_bound()
  }

  fn right_bound(&self) -> u64 {
    self.end.right_bound()
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
  pub top_left: Point,
  pub bottom_right: Point,
}

impl Rectangle {
  pub fn new(top_left: Point, bottom_right: Point) -> Rectangle {
    Rectangle {
      top_left,
      bottom_right,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
