use crate::simple_geo::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait LineMethods: Positional + Sized {
  ////////////////////////////////////////////////////////////////////////////////////////////////////
  fn start(&self) -> Point {
    self.top_left()
  }

  fn end(&self) -> Point {
    self.bottom_right()
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  fn points(&self) -> (Point, Point) {
    (self.start(), self.end())
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  fn orientation(&self) -> Orientation {
    if self.is_horizontal() {
      Horizontal
    } else {
      Vertical
    }
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  fn is_horizontal(&self) -> bool {
    self.size().is_wide()
  }

  fn is_vertical(&self) -> bool {
    self.size().is_tall()
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  fn is_parallel_to(&self, other: &Self) -> bool {
    (self.is_horizontal() && other.is_horizontal()) || (self.is_vertical() && other.is_vertical())
  }

  fn is_perpendicular_to(&self, other: &Self) -> bool {
    !self.is_parallel_to(other)
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  fn is_coaligned_with(&self, other: &Self) -> Option<Orientation> {
    if self.is_horizontally_coaligned_with(other) {
      Some(Horizontal)
    } else if self.is_vertically_coaligned_with(other) {
      Some(Vertical)
    } else {
      None
    }
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  fn is_horizontally_coaligned_with(&self, other: &Self) -> bool {
    self.is_horizontal()
      && other.is_horizontal()
      && self.length() == other.length()
      && self.start().is_left_aligned_with(other)
  }

  fn is_vertically_coaligned_with(&self, other: &Self) -> bool {
    self.is_vertical()
      && other.is_vertical()
      && self.length() == other.length()
      && self.start().is_top_aligned_with(other)
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  fn length(&self) -> usize {
    self.size().area()
  }

  fn touches(&self, rect: &Rectangle) -> bool {
    !(rect.point_is_corner(&self.start()) || rect.point_is_corner(&self.end()))
      && (self.start().overlaps(&rect.right_side())
        || self.start().overlaps(&rect.bottom_side())
        || self.end().overlaps(&rect.left_side())
        || self.end().overlaps(&rect.top_side()))
  }

  fn is_connected_to(&self, other_line: &Line) -> bool {
    self.start() == other_line.start()
      || self.start() == other_line.end()
      || self.end() == other_line.start()
      || self.end() == other_line.end()
  }
}
