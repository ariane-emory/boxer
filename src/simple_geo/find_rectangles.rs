use crate::simple_geo::*;
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////////
pub fn find_rectangles<T: LineMethods + Debug>(
  lines: Vec<T>,
  allow_overlap: bool,
) -> (Vec<Rectangle>, Vec<T>) {
  let mut rectangles: Vec<Rectangle> = Vec::new();
  let mut free_lines: Vec<T> = Vec::new();

  let mut copied_lines: Vec<T> = lines.to_vec();

  while let Some(line) = copied_lines.pop() {
    //println!("\nFind coaligned lines with    {:?}...", line);

    let mut found_a_rect = false;
    let mut lines_to_remove: Vec<T> = Vec::new();

    for other_line in &copied_lines {
      if let Some(_) = line.is_coaligned_with(other_line) {
        //println!("Found coaligned lines: \n   {:?}\n   {:?}", line,
        // other_line);

        let connected_lines: Vec<&T> = copied_lines
          .iter()
          .filter(|&tested_line| {
            line.is_connected_to(tested_line)
              && other_line.is_connected_to(tested_line)
          })
          .collect();

        match connected_lines[..] {
          [first_side, second_side] => {
            // println!(
            //   "\nWith sides:\n   {:?}\n   {:?}",
            //   first_side, second_side
            // );

            // Put the component lines in a vec and sort them so we can find
            // the top left and bottom right corners at opposite
            // ends of the vec.
            let mut tmp_vec: Vec<&T> =
              vec![&line, other_line, first_side, second_side];
            tmp_vec.sort();

            let rect =
              Rectangle::new(tmp_vec[0].start(), tmp_vec[3].end()).unwrap();

            rectangles.push(rect);

            //println!("\nNew Rectangle: {:?}", rect);

            if !allow_overlap {
              lines_to_remove.push(other_line.clone());
              lines_to_remove.push(first_side.clone());
              lines_to_remove.push(second_side.clone());
            }

            found_a_rect = true;

            break;
          }
          _ => continue,
          // _ => println!("Did not find exactly two * connecting lines."),
        }
      }
    }

    if !found_a_rect {
      //println!("No coaligned lines found for {:?}", line);
      free_lines.push(line);
    }
    else if !allow_overlap {
      for _line in lines_to_remove.iter() {
        //println!("Removing line {:?}", line);
      }
      copied_lines.retain(|l| !lines_to_remove.contains(&l));
    }
  }

  free_lines.sort();

  (rectangles, free_lines)
}
////////////////////////////////////////////////////////////////////////////////
