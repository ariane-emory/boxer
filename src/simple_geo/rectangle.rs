use crate::simple_geo::*;
use std::cmp::max;
use std::cmp::min;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rectangle {
  pub top_left: Point,
  pub bottom_right: Point,
}
////////////////////////////////////////////////////////////////////////////////
impl Rectangle {
  pub fn new(start: Point, end: Point) -> GeoResult<Self> {
    // we want the 'start' point to be the top left corner and the 'end' point
    // to be the  bottom right corner... but, they might have been passed in
    // a different order, so we're going to create our own points using the
    // minimum/maximum line and column from the arguments:
    let top_left =
      Point::new(min(start.line, end.line), min(start.col, end.col));
    let bottom_right =
      Point::new(max(start.line, end.line), max(start.col, end.col));

    if bottom_right.left_bound() - top_left.left_bound() < 2 {
      Err(ErrString::new("Rectangle must be at least 3 columns wide"))
    }
    else if bottom_right.upper_bound() - top_left.upper_bound() < 2 {
      Err(ErrString::new("Rectangle must be at least 3 lines tall"))
    }
    else {
      Ok(Self {
        top_left,
        bottom_right,
      })
    }
  }

  pub fn contained_rectangle(&self) -> Option<Self> {
    if self.top_left().line + 1 >= self.bottom_right().line
      || self.top_left().col + 1 >= self.bottom_right().col
    {
      return None;
    }

    let top_left =
      Point::new(self.top_left().line + 1, self.top_left().line + 1);
    let bottom_right =
      Point::new(self.bottom_right().line - 1, self.bottom_right().col - 1);

    Some(Self {
      top_left,
      bottom_right,
    })
  }

  pub fn contains<P: Positional>(&self, other: &P) -> bool {
    let contained_rect = self.contained_rectangle().unwrap();

    other.left_bound() >= contained_rect.left_bound()
      && other.right_bound() <= contained_rect.right_bound()
      && other.upper_bound() >= contained_rect.upper_bound()
      && other.lower_bound() <= contained_rect.lower_bound()
  }
  
  pub fn is_on_corner(&self, point: Point) -> bool {
    point == self.top_left()
      || point == self.bottom_right()
      || point == self.bottom_left()
      || point == self.top_right()
  }

  pub fn is_on_wall(&self, point: Point) -> bool {
    // If not a corner, check if the point is on one of the sides.
    !self.is_on_corner(point) &&
      (self.top_side().overlaps(&point) ||
       self.bottom_side().overlaps(&point) ||
       self.left_side().overlaps(&point) ||
       self.right_side().overlaps(&point))
  }
}
////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Rectangle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Rectangle({:?}, {:?})", self.top_left, self.bottom_right)
  }
}

////////////////////////////////////////////////////////////////////////////////
impl Positional for Rectangle {
  fn top_left(&self) -> Point {
    self.top_left
  }

  fn bottom_right(&self) -> Point {
    self.bottom_right
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Offsetable for Rectangle {
  fn offset_by(&self, line_offset: isize, col_offset: isize) -> Self {
    Self::new(
      self.top_left.offset_by(line_offset, col_offset),
      self.bottom_right.offset_by(line_offset, col_offset),
    )
    .unwrap()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Flippable for Rectangle {
  fn flip(&self) -> Self {
    Self::new(self.top_left.flip(), self.bottom_right.flip()).unwrap()
  }
}
