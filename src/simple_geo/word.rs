use crate::simple_geo::*;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Word {
  pub start: Point,
  pub end: Point,
  pub string: String,
}
////////////////////////////////////////////////////////////////////////////////
impl Word {
  pub fn new(string: &str, start: Point, end: Point) -> GeoResult<Self> {
    if start.col > end.col {
      Err(ErrString::new("start.col > end.col!"))
    }
    else if start.line != end.line {
      Err(ErrString::new("start.col > end.col!"))
    }
    else {
      Ok(Self {
        start,
        end,
        string: string.to_string(),
      })
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Word {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Word(\"{:?}\")", self.string)
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Positional for Word {
  fn top_left(&self) -> Point {
    self.start
  }

  fn bottom_right(&self) -> Point {
    self.end
  }
}
