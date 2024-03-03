use crate::simple_geo::*;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConnectedLine {
  pub start: Point,
  pub end: Point,
  pub start_connects_to: ConnectionType,
  pub end_connects_to: ConnectionType,
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConnectionType {
  Nothing,
  Corner,
  Wall,
  Variable,
}

////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for ConnectedLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let rotation_str = if self.is_vertical() { "V" } else { "H" };
    let start_connects_to_str = format!("{:?}", self.start_connects_to);
    let end_connects_to_str = format!("{:?}", self.end_connects_to);
    write!(
      f,
      "{}{:?} {:11}⇼{:11} {:?}",
      rotation_str,
      self.start,
      start_connects_to_str,
      end_connects_to_str,
      self.end
    )
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
