use crate::simple_geo::*;

////////////////////////////////////////////////////////////////////////////////
pub trait Positional:
  Clone + Eq + PartialEq + Ord + PartialOrd + Sized
{
  fn top_left(&self) -> Point;
  fn bottom_right(&self) -> Point;

  //////////////////////////////////////////////////////////////////////////////
  fn top_side(&self) -> Line {
    Line::new(
      self.top_left(),
      Point::new(self.top_left().line, self.bottom_right().col),
    )
    .unwrap()
  }

  fn bottom_side(&self) -> Line {
    Line::new(
      Point::new(self.bottom_right().line, self.top_left().col),
      self.bottom_right(),
    )
    .unwrap()
  }

  fn left_side(&self) -> Line {
    Line::new(
      self.top_left(),
      Point::new(self.bottom_right().line, self.top_left().col),
    )
    .unwrap()
  }

  fn right_side(&self) -> Line {
    Line::new(
      Point::new(self.top_left().line, self.bottom_right().col),
      self.bottom_right(),
    )
    .unwrap()
  }

  //////////////////////////////////////////////////////////////////////////////
  fn height(&self) -> usize {
    self.bottom_right().line - self.top_left().line + 1
  }

  fn width(&self) -> usize {
    self.bottom_right().col - self.top_left().col + 1
  }

  //////////////////////////////////////////////////////////////////////////////
  fn size(&self) -> Size {
    Size::new(self.height(), self.width())
  }

  //////////////////////////////////////////////////////////////////////////////
  fn area(&self) -> usize {
    self.size().area()
  }

  //////////////////////////////////////////////////////////////////////////////
  fn left_bound(&self) -> usize {
    self.top_left().col
  }

  fn right_bound(&self) -> usize {
    self.bottom_right().col
  }

  fn upper_bound(&self) -> usize {
    self.top_left().line
  }

  fn lower_bound(&self) -> usize {
    self.bottom_right().line
  }

  //////////////////////////////////////////////////////////////////////////////
  fn is_left_aligned_with(&self, other: &impl Positional) -> bool {
    self.left_bound() == other.left_bound()
  }

  fn is_right_aligned_with(&self, other: &impl Positional) -> bool {
    self.right_bound() == other.right_bound()
  }

  fn is_top_aligned_with(&self, other: &impl Positional) -> bool {
    self.upper_bound() == other.upper_bound()
  }

  fn is_bottom_aligned_with(&self, other: &impl Positional) -> bool {
    self.lower_bound() == other.lower_bound()
  }

  //////////////////////////////////////////////////////////////////////////////
  fn is_left_of(&self, other: &impl Positional) -> bool {
    self.right_bound() < other.left_bound()
  }

  fn is_right_of(&self, other: &impl Positional) -> bool {
    !(self.is_left_of(other) || self.is_left_aligned_with(other))
  }

  fn is_above(&self, other: &impl Positional) -> bool {
    self.lower_bound() < other.upper_bound()
  }

  fn is_below(&self, other: &impl Positional) -> bool {
    !(self.is_above(other) || self.is_top_aligned_with(other))
  }

  ////////////////////////////////////////////////////////////////////////////////
  fn overlaps(&self, other: &impl Positional) -> bool {
    let horizontal_overlap = self.left_bound() <= other.right_bound()
      && self.right_bound() >= other.left_bound();
    let vertical_overlap = self.upper_bound() <= other.lower_bound()
      && self.lower_bound() >= other.upper_bound();

    horizontal_overlap && vertical_overlap
  }

  ////////////////////////////////////////////////////////////////////////////////
  fn top_right(&self) -> Point {
    Point::new(self.top_left().line, self.bottom_right().col)
  }

  fn bottom_left(&self) -> Point {
    Point::new(self.bottom_right().line, self.top_left().col)
  }

  ////////////////////////////////////////////////////////////////////////////////
  fn point_is_corner(&self, point: &Point) -> bool {
    point == &self.top_left()
      || point == &self.bottom_right()
      || point == &self.top_right()
      || point == &self.bottom_left()
  }
}
