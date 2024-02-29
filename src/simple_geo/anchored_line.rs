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
    let anchoring = match self.anchoring {
      Anchoring::Start => "⇐",
      Anchoring::End => "⇐",
      Anchoring::Both => "⇔",
      Anchoring::Neither => "⤄",
    };

    let prefix = if self.is_vertical() { "V" } else { "H" };

    write!(f, "{}{:?} {} {:?}", prefix, self.start, anchoring, self.end)
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
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
    anchoring: Anchoring,
  ) -> GeoResult<Self> {
    Self::from_points(
      &Point::new(start_line, start_col),
      &Point::new(end_line, end_col),
      anchoring,
    )
  }

  pub fn from_points(start: &Point, end: &Point, anchoring: Anchoring) -> GeoResult<Self> {
    let (start, end) = Line::from_points(start, end)?.points();

    Ok(Self {
      start,
      end,
      anchoring,
    })
  }
}
