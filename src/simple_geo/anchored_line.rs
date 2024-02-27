use crate::simple_geo::*;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Anchoring {
  Start,
  End,
  Both,
  Neither,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct AnchoredLine {
  pub start: Point,
  pub end: Point,
  pub anchoring: Anchoring,
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
    anchoring: Anchoring,
  ) -> GeoResult<AnchoredLine> {
    AnchoredLine::from_points(
      &Point::new(start_col, start_line),
      &Point::new(end_col, end_line),
      anchoring,
    )
  }

  pub fn from_points(start: &Point, end: &Point, anchoring: Anchoring) -> GeoResult<Self> {
    let (start, end) = Line::from_points(start, end)?.points();

    Ok(AnchoredLine {
      start,
      end,
      anchoring,
    })
  }
}
