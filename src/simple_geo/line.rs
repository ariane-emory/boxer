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
impl LineMethods for Line {
  fn invert(&self) -> Self {
    Self::new(self.start().invert(), self.end().invert()).unwrap()
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Line {
  pub fn new(start: Point, end: Point) -> GeoResult<Self> {
    if start == end {
      return Err(ErrString::new("Start and end points cannot be the same"));
    }

    if !(start.is_left_aligned_with(&end) || start.is_top_aligned_with(&end)) {
      return Err(ErrString::new("Line must be either horizontal or vertical"));
    }

    // We want the start point to be the top/left end of the line and the end point to be
    // the bottom/right end of the line, se we might swap the arguments' order.
    let (start, end) = if (start.line < end.line) || (start.col < end.col) {
      (start, end)
    } else {
      (end, start)
    };

    Ok(Self { start, end })
  }
}
