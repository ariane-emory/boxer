#![allow(unused_variables)]

#[cfg(test)]
mod tests {
  use squares::geo::Point; // This brings `Point` into scope within the tests module

  #[test]
  fn point_test() {
    let above = Point::new(0, 0);
    let center = Point::new(4, 4);

    assert_eq!(4, center.line);
  }
}
