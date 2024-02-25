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
    Line::from_points(Point::new(0, 0), Point::new(0, 4)).unwrap(),
    Line::from_points(Point::new(5, 0), Point::new(5, 4)).unwrap(),
    Line::from_points(Point::new(0, 0), Point::new(5, 0)).unwrap(),
    Line::from_points(Point::new(0, 4), Point::new(5, 4)).unwrap(),
    Line::from_points(Point::new(3, 5), Point::new(3, 9)).unwrap(),
    Line::from_points(Point::new(0, 8), Point::new(1, 8)).unwrap(),
  ];

  lines.sort(); // Sorts in place

  for line in &lines {
    println!("{:?}", line);
  }

  // pop items off of lines until it's empty:
  while let Some(_line) = lines.pop() {}
}
////////////////////////////////////////////////////////////////////////////////////////////////////
