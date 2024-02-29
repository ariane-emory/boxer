use crate::simple_geo::*;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConnectionType {
  Nothing,
  Line,
  Input,
  Output,
  Variable,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConnectedLine {
  pub start: Point,
  pub end: Point,
  pub start_connects_to: ConnectionType,
  pub end_connects_to: ConnectionType,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for ConnectedLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?} ⇼ {:?}", self.start, self.end)
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
    start_connects_to: ConnectionType,
    end_connects_to: ConnectionType,
  ) -> GeoResult<Self> {
    Self::from_points(
      &Point::new(start_line, start_col),
      &Point::new(end_line, end_col),
      start_connects_to,
      end_connects_to,
    )
  }

  pub fn from_points(
    start: &Point,
    end: &Point,
    start_connects_to: ConnectionType,
    end_connects_to: ConnectionType,
  ) -> GeoResult<Self> {
    let (start, end) = Line::from_points(start, end)?.points();

    Ok(Self {
      start,
      end,
      start_connects_to,
      end_connects_to,
    })
  }
}
