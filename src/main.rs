#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::collections::VecDeque;
mod geo;
use geo::Orientation::*;
use geo::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
  //                   1111111111111111
  //   0123457890abcdef0123456789abcdef
  // 0 xxxxx  x    xxxxx
  // 1 x   x  x    x   x
  // 3 x   x       x   x
  // 4 x   xxxxx   x   x
  // 5 xxxxx       xxxxx

  let mut rects: Vec<Rectangle> = Vec::new();
  let mut lines = vec![
    Line::new(0, 0, 0, 5).unwrap(),
    Line::new(0, 0, 4, 0).unwrap(),
    Line::new(0, 5, 4, 5).unwrap(),
    Line::new(12, 0, 12, 5).unwrap(),
    Line::new(12, 0, 16, 0).unwrap(),
    Line::new(12, 5, 16, 5).unwrap(),
    Line::new(16, 0, 16, 5).unwrap(),
    Line::new(4, 0, 4, 5).unwrap(),
    Line::new(5, 3, 9, 3).unwrap(),
    Line::new(8, 0, 8, 1).unwrap(),
  ];

  lines.sort();
  //lines.reverse();

  for line in &lines {
    println!("{:?}", line);
  }

  let mut lines_deque = VecDeque::from(lines);

  println!("");

  // pop items off of lines until it's empty:
  while let Some(line) = lines_deque.pop_front() {
    println!("\nFind coaligned lines with {:?}...", line);

    let mut lines_to_remove = Vec::new();

    // Borrow `lines` for iteration instead of moving it
    for other_line in &lines_deque {
      if let Some(orientation) = line.is_coaligned_with(other_line) {
        match orientation {
          Horizontal => {
            println!("... Horizontally coaligned with {:?}", other_line);
            let left_candidate =
              Line::from_points(line.start.clone(), other_line.start.clone()).unwrap();
            println!("... Left side would need to match: {:?}", left_candidate);
            let right_candidate =
              Line::from_points(line.end.clone(), other_line.end.clone()).unwrap();
            println!("... Right side would need to match: {:?}", right_candidate);
          }
          Vertical => {
            println!("... Vertically coaligned with {:?}", other_line);
            let top_candidate =
              Line::from_points(line.start.clone(), other_line.start.clone()).unwrap();
            println!("... Top side would need to match: {:?}", top_candidate);
            let bottom_candidate =
              Line::from_points(line.end.clone(), other_line.end.clone()).unwrap();
            println!(
              "... Bottom side would need to match: {:?}",
              bottom_candidate
            );
          }
        }
      }
    }
    break; // temporary, we'll remove this later.
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
