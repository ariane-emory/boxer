#![allow(dead_code)]
pub mod errstring;
pub mod line;
pub mod orientation;
pub mod point;
pub mod positional;
pub mod rectangle;
pub mod size;

use crate::simple_geo::orientation::Orientation;
pub use errstring::ErrString;
pub use errstring::GeoResult;
pub use line::Line;
pub use orientation::Orientation::*;
pub use point::Point;
pub use positional::Positional;
pub use rectangle::Rectangle;
pub use size::Size;

use std::collections::VecDeque;

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
