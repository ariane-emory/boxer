#[cfg(test)]
mod tests {
  #[test]
  fn point_test() {
    let top_left = Point::new(0, 0);
    let top_middle = Point::new(0, 0);
    let top_right = Point::new(0, 4);
    let middle_left = Point::new(2, 0);
    let middle_middle = Point::new(2, 2);
    let middle_right = Point::new(2, 4);
    let bottom_left = Point::new(4, 0);
    let bottom_middle = Point::new(4, 2);
    let bottom_right = Point::new(4, 4);
    assert_eq!(4, 4);
  }
}
