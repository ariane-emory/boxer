#![allow(dead_code)]
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone)]
pub enum Orientation {
  Horizontal,
  Vertical,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
use Orientation::*;
////////////////////////////////////////////////////////////////////////////////////////////////////

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
    if self.height == 1 && self.width == 1 {
      1
    } else if self.height == 1 {
      self.width
    } else if self.width == 1 {
      self.height
    } else {
      self.height * self.width
    }
  }

  pub fn is_tall(&self) -> bool {
    self.height > self.width
  }

  pub fn is_wide(&self) -> bool {
    self.height < self.width
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Positional: Debug {
  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn left_bound(&self) -> u64;
  fn right_bound(&self) -> u64;
  fn upper_bound(&self) -> u64;
  fn lower_bound(&self) -> u64;
  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn size(&self) -> Size {
    //println!("Get size for: {:?}", self);
    let size = Size::new(
      self.lower_bound() - self.upper_bound() + 1,
      self.right_bound() - self.left_bound() + 1,
    );
    //println!("Got size:     {:?}", size);
    size
  }
  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn area(&self) -> u64 {
    self.size().area()
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn height(&self) -> u64 {
    self.size().height
  }

  fn width(&self) -> u64 {
    self.size().width
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
  pub col: u64,
  pub line: u64,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Point {
  pub fn new(col: u64, line: u64) -> Point {
    Point { col, line }
  }

  pub fn is_vertically_aligned_with(&self, other: &Self) -> bool {
    self.col == other.col
  }

  pub fn is_horizontally_aligned_with(&self, other: &Self) -> bool {
    self.line == other.line
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
type GeoResult<T> = std::result::Result<T, ErrString>;
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Line {
  pub fn from_points(start: Point, end: Point) -> GeoResult<Self> {
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

  pub fn new(start_col: u64, start_line: u64, end_col: u64, end_line: u64) -> GeoResult<Line> {
    Line::from_points(
      Point::new(start_col, start_line),
      Point::new(end_col, end_line),
    )
  }

  pub fn is_horizontal(&self) -> bool {
    self.size().is_wide()
  }

  pub fn is_vertical(&self) -> bool {
    self.size().is_tall()
  }

  pub fn is_parallel_to(&self, other: &Self) -> bool {
    (self.is_horizontal() && other.is_horizontal()) || (self.is_vertical() && other.is_vertical())
  }

  pub fn is_perpendicular_to(&self, other: &Self) -> bool {
    !self.is_parallel_to(other)
  }

  pub fn is_coaligned_with(&self, other: Self) -> Option<Orientation> {
    if self.is_horizontally_coaligned_with(other) {
      Some(Horizontal)
    } else if self.is_vertically_coaligned_with(other) {
      Some(Vertical)
    } else {
      None
    }
  }

  pub fn is_horizontally_coaligned_with(&self, other: Self) -> bool {
    self.is_horizontal()
      && other.is_horizontal()
      && self.length() == other.length()
      && self.start.is_left_aligned_with(&other)
  }

  pub fn is_vertically_coaligned_with(&self, other: Self) -> bool {
    self.is_vertical()
      && other.is_vertical()
      && self.length() == other.length()
      && self.start.is_top_aligned_with(&other)
  }

  pub fn length(&self) -> u64 {
    self.size().area()
  }
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
impl Positional for Rectangle {
  fn upper_bound(&self) -> u64 {
    self.top_left.upper_bound()
  }

  fn lower_bound(&self) -> u64 {
    self.bottom_right.lower_bound()
  }

  fn left_bound(&self) -> u64 {
    self.top_left.left_bound()
  }

  fn right_bound(&self) -> u64 {
    self.bottom_right.right_bound()
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
