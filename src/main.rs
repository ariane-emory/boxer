#![allow(dead_code)]

use std::error::Error;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
struct StringErr {
  message: String,
}
//==============================================================================
impl Error for StringErr {}
//==============================================================================
impl StringErr {
  fn new(message: &str) -> StringErr {
    StringErr {
      message: message.to_string(),
    }
  }
}
//==============================================================================
impl fmt::Display for StringErr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Line creation error: {}", self.message)
  }
}
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
type Result<T> = std::result::Result<T, StringErr>;
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
  pub line: u64,
  pub col: u64,
}
//==============================================================================
impl Point {
  fn new(line: u64, col: u64) -> Point {
    Point { line, col }
  }
}
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug)]
pub struct Line {
  pub start: Point,
  pub end: Point,
}
//==============================================================================
impl Line {
  //============================================================================
  fn is_horizontal(&self) -> bool {
    self.start.line == self.end.line
  }
  //============================================================================
  fn is_vertical(&self) -> bool {
    self.start.col == self.end.col
  }
  //============================================================================
  fn new(start: Point, end: Point) -> Result<Line> {
    if start.line != end.line && start.col != end.col {
      return Err(StringErr {
        message: "Line must be either horizontal or vertical".to_string(),
      });
    } else if start == end {
      Err(StringErr {
        message: "Start and end points cannot be the same".to_string(),
      })
    } else {
      Ok(Line { start, end })
    }
  }
  //============================================================================
}
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
  pub top_left: Point,
  pub bottom_right: Point,
}
//============================================================================
impl Rectangle {
  //==========================================================================
  fn new(top_left: Point, bottom_right: Point) -> Rectangle {
    Rectangle {
      top_left,
      bottom_right,
    }
  }
  //============================================================================
}
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////
