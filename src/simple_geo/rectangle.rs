use crate::simple_geo::*;

use std::cmp::max;
use std::cmp::min;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rectangle {
  pub top_left: Point,
  pub bottom_right: Point,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Rectangle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Rectangle({:?}, {:?})", self.top_left, self.bottom_right)
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Positional for Rectangle {
  fn top_left(&self) -> Point {
    self.top_left
  }

  fn bottom_right(&self) -> Point {
    self.bottom_right
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Rectangle {
  pub fn new(
    start_col: usize,
    start_line: usize,
    end_col: usize,
    end_line: usize,
  ) -> GeoResult<Rectangle> {
    Rectangle::from_points(
      &Point::new(start_col, start_line),
      &Point::new(end_col, end_line),
    )
  }

  pub fn from_points(start: &Point, end: &Point) -> GeoResult<Rectangle> {
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

  pub fn contained_rectangle(&self) -> Rectangle {
    let top_left = Point::new(self.top_left().line + 1, self.top_left().line + 1);
    let bottom_right = Point::new(self.bottom_right().col - 1, self.bottom_right().line - 1);
    Rectangle {
      top_left,
      bottom_right,
    }
    //Rectangle::from_points(&top_left, &bottom_right)
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
