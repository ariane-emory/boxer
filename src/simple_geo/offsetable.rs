use crate::simple_geo::*;

////////////////////////////////////////////////////////////////////////////////
pub trait Offsetable: Positional {
  //////////////////////////////////////////////////////////////////////////////
  fn flip(&self) -> Self;
  fn offset_by(&self, line_offset: isize, col_offset: isize) -> Self;
}