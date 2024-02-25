#![allow(unused_variables)]

#[cfg(test)]
mod tests {
  use squares::geo::Point;

  #[test]
  fn point_test() {
    let upper_left = Point::new(0, 0);
    let upper_middle = Point::new(0, 4);
    let upper_right = Point::new(0, 8);

    let left_of_center = Point::new(4, 0);
    let center = Point::new(4, 4);
    let right_of_center = Point::new(4, 8);

    let lower_left = Point::new(8, 0);
    let lower_middle = Point::new(8, 4);
    let lower_right = Point::new(8, 8);

    assert!(center.is_horizontally_aligned_with(upper_middle));
    assert!(center.is_horizontally_aligned_with(lower_middle));

    assert_eq!(4, center.line);
  }
}
