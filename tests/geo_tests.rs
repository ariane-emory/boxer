#![allow(unused_variables)]

#[cfg(test)]
mod tests {
  use squares::geo::Point;
  use squares::geo::Positional;

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

    assert!(!center.is_left_aligned_with(&left_of_center));
    assert!(!center.is_left_aligned_with(&lower_left));
    assert!(!center.is_left_aligned_with(&lower_right));
    assert!(!center.is_left_aligned_with(&right_of_center));
    assert!(!center.is_left_aligned_with(&upper_left));
    assert!(!center.is_left_aligned_with(&upper_right));
    assert!(center.is_left_aligned_with(&center));
    assert!(center.is_left_aligned_with(&lower_middle));
    assert!(center.is_left_aligned_with(&upper_middle));

    assert!(!center.is_right_aligned_with(&left_of_center));
    assert!(!center.is_right_aligned_with(&lower_left));
    assert!(!center.is_right_aligned_with(&lower_right));
    assert!(!center.is_right_aligned_with(&right_of_center));
    assert!(!center.is_right_aligned_with(&upper_left));
    assert!(!center.is_right_aligned_with(&upper_right));
    assert!(center.is_right_aligned_with(&center));
    assert!(center.is_right_aligned_with(&lower_middle));
    assert!(center.is_right_aligned_with(&upper_middle));

    assert!(!center.is_top_aligned_with(&lower_left));
    assert!(!center.is_top_aligned_with(&lower_middle));
    assert!(!center.is_top_aligned_with(&lower_right));
    assert!(!center.is_top_aligned_with(&upper_left));
    assert!(!center.is_top_aligned_with(&upper_middle));
    assert!(!center.is_top_aligned_with(&upper_right));
    assert!(center.is_top_aligned_with(&center));
    assert!(center.is_top_aligned_with(&left_of_center));
    assert!(center.is_top_aligned_with(&right_of_center));

    assert!(!center.is_bottom_aligned_with(&lower_left));
    assert!(!center.is_bottom_aligned_with(&lower_middle));
    assert!(!center.is_bottom_aligned_with(&lower_right));
    assert!(!center.is_bottom_aligned_with(&upper_left));
    assert!(!center.is_bottom_aligned_with(&upper_middle));
    assert!(!center.is_bottom_aligned_with(&upper_right));
    assert!(center.is_bottom_aligned_with(&center));
    assert!(center.is_bottom_aligned_with(&left_of_center));
    assert!(center.is_bottom_aligned_with(&right_of_center));

    assert!(!center.is_left_of(&center));
    assert!(!center.is_left_of(&left_of_center));
    assert!(!center.is_left_of(&lower_left));
    assert!(!center.is_left_of(&lower_middle));
    assert!(!center.is_left_of(&upper_left));
    assert!(!center.is_left_of(&upper_middle));
    assert!(center.is_left_of(&lower_right));
    assert!(center.is_left_of(&right_of_center));
    assert!(center.is_left_of(&upper_right));

    // assert!(!center.is_right_of(&center));
    // assert!(!center.is_right_of(&lower_middle));
    // assert!(!center.is_right_of(&lower_right));
    // assert!(!center.is_right_of(&right_of_center));
    // assert!(!center.is_right_of(&upper_middle));
    // assert!(!center.is_right_of(&upper_right));
    // assert!(center.is_right_of(&left_of_center));
    // assert!(center.is_right_of(&lower_left));
    // assert!(center.is_right_of(&upper_left));

    // assert!(!center.is_above(&center));
    // assert!(!center.is_above(&left_of_center));
    // assert!(!center.is_above(&right_of_center));
    // assert!(!center.is_above(&upper_left));
    // assert!(!center.is_above(&upper_middle));
    // assert!(!center.is_above(&upper_right));
    // assert!(center.is_above(&lower_left));
    // assert!(center.is_above(&lower_middle));
    // assert!(center.is_above(&lower_right));

    // assert!(!center.is_aligned_with(&lower_left));
    // assert!(!center.is_aligned_with(&lower_right));
    // assert!(!center.is_aligned_with(&upper_left));
    // assert!(!center.is_aligned_with(&upper_right));
    // assert!(center.is_aligned_with(&center));
    // assert!(center.is_aligned_with(&left_of_center));
    // assert!(center.is_aligned_with(&lower_middle));
    // assert!(center.is_aligned_with(&right_of_center));
    // assert!(center.is_aligned_with(&upper_middle));

    assert_eq!(4, center.line);
  }
}
