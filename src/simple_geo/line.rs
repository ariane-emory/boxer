use crate::simple_geo::*;
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
impl LineMethods for Line {}

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
}
