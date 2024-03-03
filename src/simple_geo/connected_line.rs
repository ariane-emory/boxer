use crate::simple_geo::*;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConnectionType {
  Nothing,
  AnotherLine,
  Wall,
  Variable,
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConnectedLine {
  pub start: Point,
  pub end: Point,
  pub start_connects_to: ConnectionType,
  pub end_connects_to: ConnectionType,
}

////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for ConnectedLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let prefix = if self.is_vertical() { "V" } else { "H" };
    write!(f, "{}{:?} ⇼ {:?}", prefix, self.start, self.end)
  }
}

////////////////////////////////////////////////////////////////////////////////
impl Positional for ConnectedLine {
  fn top_left(&self) -> Point {
    self.start
  }

  fn bottom_right(&self) -> Point {
    self.end
  }
}

////////////////////////////////////////////////////////////////////////////////
impl LineMethods for ConnectedLine {
  fn flip(&self) -> Self {
    Self::new(
      self.start().flip(),
      self.end().flip(),
      self.start_connects_to,
      self.end_connects_to,
    )
    .unwrap()
  }

  fn offset_by(&self, line_offset: isize, col_offset: isize) -> Self {
    Self::new(
      self.start.offset_by(line_offset, col_offset),
      self.end.offset_by(line_offset, col_offset),
      self.start_connects_to,
      self.end_connects_to,
    )
    .unwrap()
  }
}

////////////////////////////////////////////////////////////////////////////////
impl ConnectedLine {
  pub fn new(
    start: Point,
    end: Point,
    start_connects_to: ConnectionType,
    end_connects_to: ConnectionType,
  ) -> GeoResult<Self> {
    let (start, end) = Line::new(start, end)?.points();

    Ok(Self {
      start,
      end,
      start_connects_to,
      end_connects_to,
    })
  }
}
