#![allow(dead_code)]

use std::error::Error;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
struct ErrString {
  string: String,
}

impl ErrString {
  fn new(string: &str) -> ErrString {
    ErrString {
      string: string.to_string(),
    }
  }
}

impl Error for ErrString {}

impl fmt::Display for ErrString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error: {}", self.string)
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
  pub line: u64,
  pub col: u64,
}
impl Point {
  fn new(line: u64, col: u64) -> Point {
    Point { line, col }
  }

  fn is_horizontally_aligned_with(&self, other: Point) -> bool {
    self.line == other.line
  }

  fn is_vertically_aligned_with(&self, other: Point) -> bool {
    self.col == other.col
  }

  fn is_aligned_with(&self, other: Point) -> bool {
    self.is_horizontally_aligned_with(other) || self.is_vertically_aligned_with(other)
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug)]
pub struct Line {
  pub start: Point,
  pub end: Point,
}

type LineResult = std::result::Result<Line, ErrString>;

impl Line {
  fn is_horizontal(&self) -> bool {
    self.start.is_horizontally_aligned_with(self.end)
  }

  fn is_vertical(&self) -> bool {
    self.start.is_vertically_aligned_with(self.end)
  }

  fn new(start: Point, end: Point) -> LineResult {
    if !start.is_aligned_with(end) {
      return Err(ErrString::new("Line must be either horizontal or vertical"));
    } else if start == end {
      Err(ErrString::new("Start and end points cannot be the same"))
    } else {
      Ok(Line { start, end })
    }
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
  fn new(top_left: Point, bottom_right: Point) -> Rectangle {
    Rectangle {
      top_left,
      bottom_right,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
  //   0123457890
  // 0 xxxxx  x
  // 1 x   x  x
  // 3 x   x
  // 4 x   xxxxx
  // 5 xxxxx

  let lines = vec![
    Line::new(Point::new(0, 0), Point::new(0, 4)),
    Line::new(Point::new(5, 0), Point::new(5, 4)),
    Line::new(Point::new(0, 0), Point::new(5, 0)),
    Line::new(Point::new(0, 4), Point::new(5, 4)),
    Line::new(Point::new(3, 5), Point::new(3, 9)),
    Line::new(Point::new(0, 8), Point::new(1, 8)),
  ];

  for line in lines {
    println!("{:?}", line);
  }

  // sort lines into 3 new vectors, one containing a single Rectangle, one
  // containing a single horizontal line and one containing a single vertical line.
}
////////////////////////////////////////////////////////////////////////////////////////////////////
