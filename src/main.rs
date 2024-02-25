#[allow(dead_code)]
//   0123457890
// 0 xxxxx  x
// 1 x   x  x
// 3 x   x
// 4 x   xxxxx
// 5 xxxxx
//
// 0,0->0,4
// 5,0->5,4
// 0,0->5,0
// 0,4->5,4
//
// 3,5->3,9
// 0,8->1,8
//
#[derive(Clone, Copy, Debug)]
pub struct Point {
  pub line: u64,
  pub col: u64,
}
impl Point {
  fn new(line: u64, col: u64) -> Point {
    Point { line, col }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
  pub start: Point,
  pub end: Point,
}
impl Line {
  fn new(start: Point, end: Point) -> Line {
    Line { start, end }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Square {
  pub top_left: Point,
  pub bottom_right: Point,
}
impl Square {
  fn new(top_left: Point, bottom_right: Point) -> Square {
    Square {
      top_left,
      bottom_right,
    }
  }
}

fn main() {
  let lines = vec![
    Line::new(Point::new(0, 0), Point::new(0, 4)),
    Line::new(Point::new(5, 0), Point::new(5, 4)),
    Line::new(Point::new(0, 0), Point::new(5, 0)),
    Line::new(Point::new(0, 4), Point::new(5, 4)),
    Line::new(Point::new(3, 5), Point::new(3, 9)),
    Line::new(Point::new(0, 8), Point::new(1, 8)),
  ];

  for line in lines {
    println!("{:?}", line);
  }
}
