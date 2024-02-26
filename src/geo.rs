#![allow(dead_code)]
use std::cmp::max;
use std::cmp::min;
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
impl Error for ErrString {}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Display for ErrString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error: {}", self.string)
  }
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
type GeoResult<T> = std::result::Result<T, ErrString>;
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

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  fn overlaps(&self, other: &impl Positional) -> bool {
    let horizontal_overlap =
      self.left_bound() <= other.right_bound() && self.right_bound() >= other.left_bound();
    let vertical_overlap =
      self.upper_bound() <= other.lower_bound() && self.lower_bound() >= other.upper_bound();
    horizontal_overlap && vertical_overlap
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
  pub line: u64,
  pub col: u64,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}:{})", self.col, self.line)
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

  fn area(&self) -> u64 {
    1
  }
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

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Line {
  pub start: Point,
  pub end: Point,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Line {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?} → {:?}", self.start, self.end)
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
impl Line {
  pub fn new(start_col: u64, start_line: u64, end_col: u64, end_line: u64) -> GeoResult<Line> {
    Line::from_points(
      &Point::new(start_col, start_line),
      &Point::new(end_col, end_line),
    )
  }

  pub fn from_points(start: &Point, end: &Point) -> GeoResult<Self> {
    if start == end {
      return Err(ErrString::new("Start and end points cannot be the same"));
    }

    if !(start.is_left_aligned_with(end) || start.is_top_aligned_with(end)) {
      return Err(ErrString::new("Line must be either horizontal or vertical"));
    }
    // We want the start point to be the top/left end of the line and the end point to be
    // the bottom/right end of the line, se we might swap the arguments' order.
    let (start, end) = if (start.line < end.line) || (start.col < end.col) {
      (start, end)
    } else {
      (end, start)
    };
    Ok(Line {
      start: *start,
      end: *end,
    })
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

  pub fn is_coaligned_with(&self, other: &Self) -> Option<Orientation> {
    if self.is_horizontally_coaligned_with(other) {
      Some(Horizontal)
    } else if self.is_vertically_coaligned_with(other) {
      Some(Vertical)
    } else {
      None
    }
  }

  pub fn is_horizontally_coaligned_with(&self, other: &Self) -> bool {
    self.is_horizontal()
      && other.is_horizontal()
      && self.length() == other.length()
      && self.start.is_left_aligned_with(other)
  }

  pub fn is_vertically_coaligned_with(&self, other: &Self) -> bool {
    self.is_vertical()
      && other.is_vertical()
      && self.length() == other.length()
      && self.start.is_top_aligned_with(other)
  }

  pub fn length(&self) -> u64 {
    self.size().area()
  }

  pub fn orientation(&self) -> Orientation {
    if self.is_horizontal() {
      Horizontal
    } else {
      Vertical
    }
  }

  pub fn touches(&self, rectangle: &Rectangle) -> bool {
    if self.is_horizontal() {
      // For a horizontal line, check if it's adjacent to the left or right of the rectangle
      (self.start.col == rectangle.right_bound() + 1 || self.end.col == rectangle.left_bound() - 1)
        && self.start.line >= rectangle.upper_bound()
        && self.start.line <= rectangle.lower_bound()
    } else if self.is_vertical() {
      // For a vertical line, check if it's adjacent above or below the rectangle
      (self.start.line == rectangle.lower_bound() + 1
        || self.start.line == rectangle.upper_bound() - 1)
        && self.start.col >= rectangle.left_bound()
        && self.start.col <= rectangle.right_bound()
    } else {
      false
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
  pub fn new(start: Point, end: Point) -> GeoResult<Rectangle> {
    // we want the 'start' point to be the top left corner and the 'end' point to be the  bottom
    // right corner... but, they might have been passed in a different order, so we're going to
    // create our own points using the minimum/maximum line and column from the arguments:
    let top_left = Point::new(min(start.col, end.col), min(start.line, end.line));
    let bottom_right = Point::new(max(start.col, end.col), max(start.line, end.line));

    if bottom_right.left_bound() - top_left.left_bound() < 2 {
      Err(ErrString::new("Rectangle must be at least 3 columns wide"))
    } else if bottom_right.upper_bound() - top_left.upper_bound() < 2 {
      Err(ErrString::new("Rectangle must be at least 3 lines tall"))
    } else {
      Ok(Rectangle {
        top_left,
        bottom_right,
      })
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
