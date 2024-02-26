use crate::simple_geo::ErrString;
use crate::simple_geo::GeoResult;
use crate::simple_geo::Orientation;
use crate::simple_geo::Orientation::*;
use crate::simple_geo::Point;
use crate::simple_geo::Positional;
use crate::simple_geo::Rectangle;
use std::fmt;

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
  fn top_left(&self) -> Point {
    self.start
  }

  fn bottom_right(&self) -> Point {
    self.end
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

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  pub fn orientation(&self) -> Orientation {
    if self.is_horizontal() {
      Horizontal
    } else {
      Vertical
    }
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  pub fn is_horizontal(&self) -> bool {
    self.size().is_wide()
  }

  pub fn is_vertical(&self) -> bool {
    self.size().is_tall()
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  pub fn is_parallel_to(&self, other: &Self) -> bool {
    (self.is_horizontal() && other.is_horizontal()) || (self.is_vertical() && other.is_vertical())
  }

  pub fn is_perpendicular_to(&self, other: &Self) -> bool {
    !self.is_parallel_to(other)
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  pub fn is_coaligned_with(&self, other: &Self) -> Option<Orientation> {
    if self.is_horizontally_coaligned_with(other) {
      Some(Horizontal)
    } else if self.is_vertically_coaligned_with(other) {
      Some(Vertical)
    } else {
      None
    }
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
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

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  pub fn length(&self) -> usize {
    self.size().area()
  }

  pub fn touches(&self, rect: &Rectangle) -> bool {
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
