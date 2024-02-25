mod geo;
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

  let mut lines = vec![
    Line::new(0, 0, 4, 0).unwrap(),
    Line::new(0, 5, 4, 5).unwrap(),
    Line::new(0, 0, 0, 5).unwrap(),
    Line::new(4, 0, 4, 5).unwrap(),
    Line::new(5, 3, 9, 3).unwrap(),
    Line::new(8, 0, 8, 1).unwrap(),
  ];

  lines.sort(); // Sorts in place

  for line in &lines {
    println!("{:?}", line);
  }

  // pop items off of lines until it's empty:
  while let Some(_line) = lines.pop() {}
}
////////////////////////////////////////////////////////////////////////////////////////////////////
