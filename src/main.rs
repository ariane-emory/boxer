#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::collections::VecDeque;
mod geo;
use geo::Orientation::*;
use geo::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
  // //                   1111111111111111
  // //   0123457890abcdef0123456789abcdef
  // // 0 xxxxx  x    xxxxx
  // // 1 x   x  x    x   x
  // // 3 x   x       x   x
  // // 4 x   xxxxxx  x   x
  // // 5 xxxxx       xxxxx

  // let mut lines = vec![
  //   Line::new(0, 0, 0, 5).unwrap(),
  //   Line::new(0, 0, 4, 0).unwrap(),
  //   Line::new(0, 5, 4, 5).unwrap(),
  //   Line::new(12, 0, 12, 5).unwrap(),
  //   Line::new(12, 0, 16, 0).unwrap(),
  //   Line::new(12, 5, 16, 5).unwrap(),
  //   Line::new(16, 0, 16, 5).unwrap(),
  //   Line::new(4, 0, 4, 5).unwrap(),
  //   Line::new(5, 4, 10, 4).unwrap(),
  //   Line::new(8, 0, 8, 1).unwrap(),
  // ];

  //                   1111111111111111
  //   0123457890abcdef0123456789abcdef
  // 0 xxxxxxx
  // 1 x     x
  // 3 x     x
  // 4 x  xxxxxxx
  // 5 x  x  x  x
  // 6 x  x  x  x
  // 7 xxxxxxx  x
  // 8    x     x
  // 9    xxxxxxx

  let mut lines = vec![
    Line::new(0, 0, 7, 0).unwrap(),
    Line::new(3, 4, 16, 4).unwrap(),
    Line::new(0, 7, 7, 7).unwrap(),
    Line::new(3, 9, 16, 9).unwrap(),
    Line::new(0, 0, 0, 7).unwrap(),
    Line::new(7, 0, 7, 7).unwrap(),
    Line::new(3, 4, 3, 9).unwrap(),
    Line::new(16, 4, 16, 9).unwrap(),
  ];

  lines.sort();

  for line in &lines {
    println!("Input Line: {:?}", line);
  }

  let mut lines_deque: VecDeque<Line> = VecDeque::from(lines);
  let mut leftover_lines = Vec::new();
  let mut rects: Vec<Rectangle> = Vec::new();

  while let Some(line) = lines_deque.pop_front() {
    println!("\nFind coaligned lines with {:?}...", line);

    let mut found_a_rect = false;
    let mut lines_to_remove: Vec<Line> = Vec::new();

    for other_line in &lines_deque {
      if let Some(orientation) = line.is_coaligned_with(other_line) {
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
            println!("With sides:\n   {:?}\n   {:?}", first_side, second_side);

            // Put the component lines in a vec and sort them so we can find the top left and bottom right
            // corners at opposite ends of the vec.
            let mut tmp_vec: Vec<&Line> = vec![&line, other_line, first_side, second_side];
            tmp_vec.sort();

            let rect = Rectangle::from_points(&tmp_vec[0].start, &tmp_vec[3].end).unwrap();

            rects.push(rect);

            println!("New Rectangle: {:?}", rect);

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

  for rect in &rects {
    println!("Discovered Rectangle: {:?}", rect);
  }

  for line in &leftover_lines {
    println!("Leftover Line: {:?}", line);
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
