use crate::simple_geo::*;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct AnchoredLine {
  pub start: Point,
  pub end: Point,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for AnchoredLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?} → {:?}", self.start, self.end)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Positional for AnchoredLine {
  fn top_left(&self) -> Point {
    self.start
  }

  fn bottom_right(&self) -> Point {
    self.end
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl LineMethods for AnchoredLine {}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl AnchoredLine {
  pub fn new(
    start_col: usize,
    start_line: usize,
    end_col: usize,
    end_line: usize,
  ) -> GeoResult<AnchoredLine> {
    AnchoredLine::from_points(
      &Point::new(start_col, start_line),
      &Point::new(end_col, end_line),
    )
  }

  pub fn from_points(start: &Point, end: &Point) -> GeoResult<Self> {
    let line = Line::from_points(start, end);

    let (start, end) = (line.start, line.end);

    Ok(AnchoredLine { start, end })
  }
}
