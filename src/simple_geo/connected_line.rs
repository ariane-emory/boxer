use crate::simple_geo::*;
use std::fmt;
use ConnectionType::*;

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConnectionType {
  Nothing,
  Corner,
  Wall,
}
////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConnectedLine {
  pub orientation: Orientation,
  pub start: Point,
  pub end: Point,
  pub start_connects_to: ConnectionType,
  pub end_connects_to: ConnectionType,
}
////////////////////////////////////////////////////////////////////////////////
impl ConnectedLine {
  pub fn new(
    orientation: Orientation,
    start: Point,
    end: Point,
    start_connects_to: ConnectionType,
    end_connects_to: ConnectionType,
  ) -> GeoResult<Self> {
    let (start, end) = Line::new(start, end)?.points();

    Ok(Self {
      start,
      end,
      orientation,
      start_connects_to,
      end_connects_to,
    })
  }

  pub fn corner_connected(&self) -> bool {
    self.start_connects_to == Corner && self.end_connects_to == Corner
  }
}
////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for ConnectedLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let orientation_str = match self.orientation {
      Orientation::Horizontal => "H",
      Orientation::Vertical => "V",
    };
    let connection_str = format!(
      "{:?}←{}→{:?}",
      self.start_connects_to,
      self.len(),
      self.end_connects_to
    );
    write!(
      f,
      "{}{:?} {:15}{:?}",
      orientation_str, self.start, connection_str, self.end
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
impl LineMethods for ConnectedLine {}
////////////////////////////////////////////////////////////////////////////////
impl Offsetable for ConnectedLine {
  fn flip(&self) -> Self {
    Self::new(
      self.orientation,
      self.start().flip(),
      self.end().flip(),
      self.start_connects_to,
      self.end_connects_to,
    )
    .unwrap()
  }

  fn offset_by(&self, line_offset: isize, col_offset: isize) -> Self {
    Self::new(
      self.orientation,
      self.start.offset_by(line_offset, col_offset),
      self.end.offset_by(line_offset, col_offset),
      self.start_connects_to,
      self.end_connects_to,
    )
    .unwrap()
  }
}
