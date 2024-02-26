#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
mod simple_geo;
use simple_geo::*;

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

  let mut leftover_lines = Vec::new();
  let mut rects = Vec::new();

  find_rectangles(&lines, &mut rects, &mut leftover_lines);

  for rect in &rects {
    println!("Discovered Rectangle: {:?}", rect);
  }

  for line in &leftover_lines {
    println!("Leftover Line: {:?}", line);
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
