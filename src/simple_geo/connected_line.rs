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
pub struct ConnectedLine {
  pub start: Point,
  pub end: Point,
  pub anchoring: Anchoring,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for ConnectedLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let anchoring = match self.anchoring {
      Anchoring::Start => "⇐",
      Anchoring::End => "⇐",
      Anchoring::Both => "⇔",
      Anchoring::Neither => "⤄",
    };

    write!(f, "{:?} {} {:?}", self.start, anchoring, self.end)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Positional for ConnectedLine {
  fn top_left(&self) -> Point {
    self.start
  }

  fn bottom_right(&self) -> Point {
    self.end
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl LineMethods for ConnectedLine {}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl ConnectedLine {
  pub fn new(
    start_col: usize,
    start_line: usize,
    end_col: usize,
    end_line: usize,
    anchoring: Anchoring,
  ) -> GeoResult<Self> {
    Self::from_points(
      &Point::new(start_col, start_line),
      &Point::new(end_col, end_line),
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
