#![allow(unused_variables)]

#[cfg(test)]
mod tests {
  use squares::geo::Point;

  #[test]
  fn point_test() {
    let above = Point::new(0, 0);
    let center = Point::new(4, 4);
    let below = Point::new(8, 8);
    let left_of_center = Point::new(4, 0);
    let right_of_center = Point::new(4, 8);
    let upper_left = Point::new(0, 0);
    let upper_right = Point::new(0, 8);
    let lower_left = Point::new(8, 0);
    let lower_right = Point::new(8, 8);

    assert_eq!(4, center.line);
  }
}
