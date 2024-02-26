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
  pub height: usize,
  pub width: usize,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Rectangular for Size {
  fn top_left(&self) -> Point {
    Point::new(0, 0)
  }

  fn bottom_right(&self) -> Point {
    Point::new(self.width - 1, self.height - 1)
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Size {
  pub fn new(height: usize, width: usize) -> Size {
    Size { height, width }
  }

  pub fn area(&self) -> usize {
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
pub trait Rectangular {
  fn top_left(&self) -> Point;

  fn top_right(&self) -> Point {
    Point::new(self.bottom_right().col, self.top_left().line)
  }

  fn bottom_left(&self) -> Point {
    Point::new(self.top_left().col, self.bottom_right().line)
  }

  fn bottom_right(&self) -> Point;

  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn top_side(&self) -> Line {
    Line::new(
      self.top_left().col,
      self.top_left().line,
      self.bottom_right().col,
      self.top_left().line,
    )
    .unwrap()
  }

  fn bottom_side(&self) -> Line {
    Line::new(
      self.top_left().col,
      self.bottom_right().line,
      self.bottom_right().col,
      self.bottom_right().line,
    )
    .unwrap()
  }

  fn left_side(&self) -> Line {
    Line::new(
      self.top_left().col,
      self.top_left().line,
      self.top_left().col,
      self.bottom_right().line,
    )
    .unwrap()
  }

  fn right_side(&self) -> Line {
    Line::new(
      self.bottom_right().col,
      self.top_left().line,
      self.bottom_right().col,
      self.bottom_right().line,
    )
    .unwrap()
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn point_is_corner(&self, point: &Point) -> bool {
    point == &self.top_left()
      || point == &self.bottom_right()
      || point == &self.top_right()
      || point == &self.bottom_left()
  }

  fn contained_area(&self) -> GeoResult<Box> {
    let top_left = Point::new(self.top_left().col + 1, self.top_left().line + 1);
    let bottom_right = Point::new(self.bottom_right().col - 1, self.bottom_right().line - 1);
    Box::from_points(&top_left, &bottom_right)
  }

  fn size(&self) -> Size {
    Size::new(
      self.bottom_right().line - self.top_left().line + 1,
      self.bottom_right().col - self.top_left().col + 1,
    )
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn height(&self) -> usize {
    self.size().height
  }

  fn width(&self) -> usize {
    self.size().width
  }

  fn area(&self) -> usize {
    self.size().area()
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Positional: Rectangular {
  fn left_bound(&self) -> usize;
  fn right_bound(&self) -> usize;
  fn upper_bound(&self) -> usize;
  fn lower_bound(&self) -> usize;

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
  pub line: usize,
  pub col: usize,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}:{})", self.col, self.line)
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Positional for Point {
  fn upper_bound(&self) -> usize {
    self.line
  }

  fn lower_bound(&self) -> usize {
    self.line
  }

  fn left_bound(&self) -> usize {
    self.col
  }

  fn right_bound(&self) -> usize {
    self.col
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Rectangular for Point {
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
impl Rectangular for Line {
  fn top_left(&self) -> Point {
    self.start
  }

  fn bottom_right(&self) -> Point {
    self.end
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Positional for Line {
  fn upper_bound(&self) -> usize {
    self.start.upper_bound()
  }

  fn lower_bound(&self) -> usize {
    self.end.lower_bound()
  }

  fn left_bound(&self) -> usize {
    self.start.left_bound()
  }

  fn right_bound(&self) -> usize {
    self.end.right_bound()
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Line {
  pub fn new(
    start_col: usize,
    start_line: usize,
    end_col: usize,
    end_line: usize,
  ) -> GeoResult<Line> {
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

  pub fn length(&self) -> usize {
    self.size().area()
  }

  pub fn orientation(&self) -> Orientation {
    if self.is_horizontal() {
      Horizontal
    } else {
      Vertical
    }
  }

  pub fn touches(&self, rect: &Box) -> bool {
    !(rect.point_is_corner(&self.start) || rect.point_is_corner(&self.end))
      && (self.start.overlaps(&rect.right_side())
        || self.start.overlaps(&rect.bottom_side())
        || self.end.overlaps(&rect.left_side())
        || self.end.overlaps(&rect.top_side()))
  }

  pub fn is_connected_to(&self, other_line: &Line) -> bool {
    self.start == other_line.start
      || self.start == other_line.end
      || self.end == other_line.start
      || self.end == other_line.end
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Box {
  pub top_left: Point,
  pub bottom_right: Point,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Box {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Box({:?}, {:?})", self.top_left, self.bottom_right)
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Positional for Box {
  fn upper_bound(&self) -> usize {
    self.top_left.upper_bound()
  }

  fn lower_bound(&self) -> usize {
    self.bottom_right.lower_bound()
  }

  fn left_bound(&self) -> usize {
    self.top_left.left_bound()
  }

  fn right_bound(&self) -> usize {
    self.bottom_right.right_bound()
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Rectangular for Box {
  fn top_left(&self) -> Point {
    self.top_left
  }

  fn bottom_right(&self) -> Point {
    self.bottom_right
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Box {
  pub fn new(
    start_col: usize,
    start_line: usize,
    end_col: usize,
    end_line: usize,
  ) -> GeoResult<Box> {
    Box::from_points(
      &Point::new(start_col, start_line),
      &Point::new(end_col, end_line),
    )
  }

  pub fn from_points(start: &Point, end: &Point) -> GeoResult<Box> {
    // we want the 'start' point to be the top left corner and the 'end' point to be the  bottom
    // right corner... but, they might have been passed in a different order, so we're going to
    // create our own points using the minimum/maximum line and column from the arguments:
    let top_left = Point::new(min(start.col, end.col), min(start.line, end.line));
    let bottom_right = Point::new(max(start.col, end.col), max(start.line, end.line));

    if bottom_right.left_bound() - top_left.left_bound() < 2 {
      Err(ErrString::new("Box must be at least 3 columns wide"))
    } else if bottom_right.upper_bound() - top_left.upper_bound() < 2 {
      Err(ErrString::new("Box must be at least 3 lines tall"))
    } else {
      Ok(Box {
        top_left,
        bottom_right,
      })
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
