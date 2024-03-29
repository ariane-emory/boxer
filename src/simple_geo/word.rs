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
      Err(ErrString::new("start.line != end.line!"))
    }
    else {
      Ok(Self {
        string: string.to_string(),
        start,
        end,
      })
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Word {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Word({:?} → {:?} {:?})", self.start, self.end, self.string)
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
////////////////////////////////////////////////////////////////////////////////
impl Offsetable for Word {
  fn offset_by(&self, line_offset: isize, col_offset: isize) -> Self {
    Self::new(
      &self.string,
      self.start.offset_by(line_offset, col_offset),
      self.end.offset_by(line_offset, col_offset),
    )
    .unwrap()
  }
}
////////////////////////////////////////////////////////////////////////////////
// impl Flippable for Word {
//   fn flip(&self) -> Self {
//     Self::new(&self.string, self.start.flip(), self.end.flip()).unwrap()
//   }
// }
