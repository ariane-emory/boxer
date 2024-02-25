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
  // 4 x   xxxxxx  x   x
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
    Line::new(5, 4, 10, 4).unwrap(),
    Line::new(8, 0, 8, 1).unwrap(),
  ];

  lines.sort();
  //lines.reverse();

  for line in &lines {
    println!("Soure Line: {:?}", line);
  }

  let mut lines_deque = VecDeque::from(lines);
  let mut leftover_lines = Vec::new();

  println!("");

  // pop items off of lines until it's empty:
  while let Some(line) = lines_deque.pop_front() {
    println!("\nFind coaligned lines with {:?}...", line);

    let mut lines_to_remove = Vec::new();
    let mut found_a_rect = false;

    // Borrow `lines` for iteration instead of moving it
    for other_line in &lines_deque {
      if let Some(orientation) = line.is_coaligned_with(other_line) {
        let (left_or_top_candidate, right_or_bottom_candidate) = match orientation {
          Horizontal => (
            Line::from_points(line.start.clone(), other_line.start.clone()).unwrap(),
            Line::from_points(line.end.clone(), other_line.end.clone()).unwrap(),
          ),
          Vertical => (
            Line::from_points(line.start.clone(), other_line.start.clone()).unwrap(),
            Line::from_points(line.end.clone(), other_line.end.clone()).unwrap(),
          ),
        };

        // Check if the candidate lines are in the deque
        if lines_deque.contains(&left_or_top_candidate)
          && lines_deque.contains(&right_or_bottom_candidate)
        {
          found_a_rect = true;
          println!("Found coaligned lines: \n  {:?}\n  {:?}", line, other_line);
          println!(
            "With sides:\n  {:?}\n  {:?}",
            left_or_top_candidate, right_or_bottom_candidate
          );

          // Put the component lines in a vec and sort then so we can find the top left and bottom right
          // corners at opposite ends of the vec.
          let mut tmp_vec = Vec::new();
          tmp_vec.push(line.clone());
          tmp_vec.push(other_line.clone());
          tmp_vec.push(left_or_top_candidate.clone());
          tmp_vec.push(right_or_bottom_candidate.clone());
          tmp_vec.sort();

          for t in &tmp_vec {
            println!("-> {:?}", t);
          }

          // Create the rectangle here...
          let rect = Rectangle::new(tmp_vec.first().unwrap().start, tmp_vec.last().unwrap().end);

          println!("New Rectangle: {:?}", rect);

          // Schedule the lines for removal
          lines_to_remove.push(other_line.clone());
          lines_to_remove.push(left_or_top_candidate);
          lines_to_remove.push(right_or_bottom_candidate);

          rects.push(rect);

          lines_deque.retain(|l| !lines_to_remove.contains(l));

          break;
        }
      }
    }

    if !found_a_rect {
      println!("No coaligned lines found for {:?}", line);
      leftover_lines.push(line.clone());
    }
  }

  for rect in &rects {
    println!("Discovered Rectangle: {:?}", rect);
  }

  for line in &lines_deque {
    println!("Leftover Line: {:?}", line);
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
