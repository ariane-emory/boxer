mod geo;
use geo::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
  //   0123457890
  // 0 xxxxx  x
  // 1 x   x  x
  // 3 x   x
  // 4 x   xxxxx
  // 5 xxxxx

  let mut lines = vec![
    Line::new(Point::new(0, 0), Point::new(0, 4)).unwrap(),
    Line::new(Point::new(5, 0), Point::new(5, 4)).unwrap(),
    Line::new(Point::new(0, 0), Point::new(5, 0)).unwrap(),
    Line::new(Point::new(0, 4), Point::new(5, 4)).unwrap(),
    Line::new(Point::new(3, 5), Point::new(3, 9)).unwrap(),
    Line::new(Point::new(0, 8), Point::new(1, 8)).unwrap(),
  ];

  lines.sort(); // Sorts in place

  for line in &lines {
    println!("{:?}", line);
  }

  // // TODO: sort lines into 3 new vectors, one containing a single Rectangle, one
  // // containing a single horizontal line and one containing a single vertical line.
  // // Initialization
  // let mut rectangles = Vec::new();
  // let mut vertical_lines = Vec::new();
  // let mut horizontal_lines = Vec::new();

  // // Step 2: Identification of RectanglesT
  // for i in 0..lines.len() {
  //   let line1 = &lines[i];
  //   if line1.is_horizontal() {
  //     for j in i + 1..lines.len() {
  //       let line2 = &lines[j];
  //       if line2.is_horizontal() && line1.is_parallel_to(line2) {
  //         // Check for connecting vertical lines
  //         if let Some((v1, v2)) = find_connecting_vertical_lines(line1, line2, &lines) {
  //           rectangles.push(Rectangle::new(line1.start, line2.end));
  //           // Remove or mark these lines so they are not considered in the next step
  //           break;
  //         }
  //       }
  //     }
  //   }
  // }

  // // Step 3: Categorization of Remaining Lines
  // for line in lines {
  //   if !line.is_part_of_any_rectangle() {
  //     if line.is_horizontal() {
  //       horizontal_lines.push(line);
  //     } else if line.is_vertical() {
  //       vertical_lines.push(line);
  //     }
  //   }
  // }

  // Now `rectangles`, `vertical_lines`, and `horizontal_lines` hold the categorized lines.
}
////////////////////////////////////////////////////////////////////////////////////////////////////
