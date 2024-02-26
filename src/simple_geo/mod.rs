#![allow(dead_code)]
pub mod errstring;
pub mod point;
pub mod rectangle;
pub mod size;

pub use errstring::ErrString;
pub use errstring::GeoResult;
pub use point::Point;
pub use rectangle::Rectangle;
pub use size::Size;

use std::collections::VecDeque;
use std::fmt;
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone)]
pub enum Orientation {
  Horizontal,
  Vertical,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
use Orientation::*;
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Positional {
  fn top_left(&self) -> Point;

  fn bottom_right(&self) -> Point;

  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn top_side(&self) -> Line {
    Line::new(
      self.top_left().col,
      self.top_left().line,
      self.bottom_right().col,
      self.top_left().line,
    )
    .unwrap()
  }

  fn bottom_side(&self) -> Line {
    Line::new(
      self.top_left().col,
      self.bottom_right().line,
      self.bottom_right().col,
      self.bottom_right().line,
    )
    .unwrap()
  }

  fn left_side(&self) -> Line {
    Line::new(
      self.top_left().col,
      self.top_left().line,
      self.top_left().col,
      self.bottom_right().line,
    )
    .unwrap()
  }

  fn right_side(&self) -> Line {
    Line::new(
      self.bottom_right().col,
      self.top_left().line,
      self.bottom_right().col,
      self.bottom_right().line,
    )
    .unwrap()
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn height(&self) -> usize {
    self.bottom_right().line - self.top_left().line + 1
  }

  fn width(&self) -> usize {
    self.bottom_right().col - self.top_left().col + 1
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn size(&self) -> Size {
    Size::new(self.height(), self.width())
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn area(&self) -> usize {
    self.size().area()
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
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

  //////////////////////////////////////////////////////////////////////////////////////////////////
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

  //////////////////////////////////////////////////////////////////////////////////////////////////
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

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  fn overlaps(&self, other: &impl Positional) -> bool {
    let horizontal_overlap =
      self.left_bound() <= other.right_bound() && self.right_bound() >= other.left_bound();
    let vertical_overlap =
      self.upper_bound() <= other.lower_bound() && self.lower_bound() >= other.upper_bound();
    horizontal_overlap && vertical_overlap
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  fn top_right(&self) -> Point {
    Point::new(self.bottom_right().col, self.top_left().line)
  }

  fn bottom_left(&self) -> Point {
    Point::new(self.top_left().col, self.bottom_right().line)
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  fn point_is_corner(&self, point: &Point) -> bool {
    point == &self.top_left()
      || point == &self.bottom_right()
      || point == &self.top_right()
      || point == &self.bottom_left()
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Line {
  pub start: Point,
  pub end: Point,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Line {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?} → {:?}", self.start, self.end)
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Positional for Line {
  fn top_left(&self) -> Point {
    self.start
  }

  fn bottom_right(&self) -> Point {
    self.end
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Line {
  pub fn new(
    start_col: usize,
    start_line: usize,
    end_col: usize,
    end_line: usize,
  ) -> GeoResult<Line> {
    Line::from_points(
      &Point::new(start_col, start_line),
      &Point::new(end_col, end_line),
    )
  }

  pub fn from_points(start: &Point, end: &Point) -> GeoResult<Self> {
    if start == end {
      return Err(ErrString::new("Start and end points cannot be the same"));
    }

    if !(start.is_left_aligned_with(end) || start.is_top_aligned_with(end)) {
      return Err(ErrString::new("Line must be either horizontal or vertical"));
    }
    // We want the start point to be the top/left end of the line and the end point to be
    // the bottom/right end of the line, se we might swap the arguments' order.
    let (start, end) = if (start.line < end.line) || (start.col < end.col) {
      (start, end)
    } else {
      (end, start)
    };
    Ok(Line {
      start: *start,
      end: *end,
    })
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  pub fn orientation(&self) -> Orientation {
    if self.is_horizontal() {
      Horizontal
    } else {
      Vertical
    }
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  pub fn is_horizontal(&self) -> bool {
    self.size().is_wide()
  }

  pub fn is_vertical(&self) -> bool {
    self.size().is_tall()
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  pub fn is_parallel_to(&self, other: &Self) -> bool {
    (self.is_horizontal() && other.is_horizontal()) || (self.is_vertical() && other.is_vertical())
  }

  pub fn is_perpendicular_to(&self, other: &Self) -> bool {
    !self.is_parallel_to(other)
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  pub fn is_coaligned_with(&self, other: &Self) -> Option<Orientation> {
    if self.is_horizontally_coaligned_with(other) {
      Some(Horizontal)
    } else if self.is_vertically_coaligned_with(other) {
      Some(Vertical)
    } else {
      None
    }
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  pub fn is_horizontally_coaligned_with(&self, other: &Self) -> bool {
    self.is_horizontal()
      && other.is_horizontal()
      && self.length() == other.length()
      && self.start.is_left_aligned_with(other)
  }

  pub fn is_vertically_coaligned_with(&self, other: &Self) -> bool {
    self.is_vertical()
      && other.is_vertical()
      && self.length() == other.length()
      && self.start.is_top_aligned_with(other)
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////
  pub fn length(&self) -> usize {
    self.size().area()
  }

  pub fn touches(&self, rect: &Rectangle) -> bool {
    !(rect.point_is_corner(&self.start) || rect.point_is_corner(&self.end))
      && (self.start.overlaps(&rect.right_side())
        || self.start.overlaps(&rect.bottom_side())
        || self.end.overlaps(&rect.left_side())
        || self.end.overlaps(&rect.top_side()))
  }

  pub fn is_connected_to(&self, other_line: &Line) -> bool {
    self.start == other_line.start
      || self.start == other_line.end
      || self.end == other_line.start
      || self.end == other_line.end
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
// Free functions
////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn find_rectangles(
  lines: &Vec<Line>,
  rects: &mut Vec<Rectangle>,
  leftover_lines: &mut Vec<Line>,
) {
  let mut sorted_lines = lines.clone();
  sorted_lines.sort();

  let mut lines_deque: VecDeque<Line> = VecDeque::from(sorted_lines);

  while let Some(line) = lines_deque.pop_front() {
    println!("\nFind coaligned lines with {:?}...", line);

    let mut found_a_rect = false;
    let mut lines_to_remove: Vec<Line> = Vec::new();

    for other_line in &lines_deque {
      if let Some(_) = line.is_coaligned_with(other_line) {
        println!(
          "Found coaligned lines: \n   {:?}\n   {:?}",
          line, other_line
        );

        let connected_lines: Vec<&Line> = lines_deque
          .iter()
          .filter(|&tested_line| {
            line.is_connected_to(tested_line) && other_line.is_connected_to(tested_line)
          })
          .collect();

        match connected_lines[..] {
          [first_side, second_side] => {
            println!("\nWith sides:\n   {:?}\n   {:?}", first_side, second_side);

            // Put the component lines in a vec and sort them so we can find the top left
            // and bottom right corners at opposite ends of the vec.
            let mut tmp_vec: Vec<&Line> = vec![&line, other_line, first_side, second_side];
            tmp_vec.sort();

            let rect = Rectangle::from_points(&tmp_vec[0].start, &tmp_vec[3].end).unwrap();

            rects.push(rect);

            println!("\nNew Rectangle: {:?}", rect);

            lines_to_remove.push(*other_line);
            lines_to_remove.push(*first_side);
            lines_to_remove.push(*second_side);

            found_a_rect = true;

            break;
          }
          _ => println!("Did not find exactly two connecting lines."),
        }
      }
    }

    if !found_a_rect {
      println!("No coaligned lines found for {:?}", line);
      leftover_lines.push(line.clone());
    } else {
      lines_deque.retain(|l| !lines_to_remove.contains(&l));
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
