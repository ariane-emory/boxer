#![allow(unused_variables)]

#[cfg(test)]
mod tests {
  use squares::geo::*;
  // use squares::geo::Point;
  // use squares::geo::Positional;

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

    assert_eq!(center.size(), Size::new(1, 1));
    assert_eq!(center.size().area(), 1);

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

    assert!(!center.is_right_of(&center));
    assert!(!center.is_right_of(&lower_middle));
    assert!(!center.is_right_of(&lower_right));
    assert!(!center.is_right_of(&right_of_center));
    assert!(!center.is_right_of(&upper_middle));
    assert!(!center.is_right_of(&upper_right));
    assert!(center.is_right_of(&left_of_center));
    assert!(center.is_right_of(&lower_left));
    assert!(center.is_right_of(&upper_left));

    assert!(!center.is_above(&center));
    assert!(!center.is_above(&left_of_center));
    assert!(!center.is_above(&right_of_center));
    assert!(!center.is_above(&upper_left));
    assert!(!center.is_above(&upper_middle));
    assert!(!center.is_above(&upper_right));
    assert!(center.is_above(&lower_left));
    assert!(center.is_above(&lower_middle));
    assert!(center.is_above(&lower_right));
  }

  #[test]
  fn line_test() {
    let upper_horizontal = Line::new(Point::new(0, 0), Point::new(0, 4)).unwrap();
    let lower_horizontal = Line::new(Point::new(0, 0), Point::new(0, 4)).unwrap();
    let offset_horizontal = Line::new(Point::new(0, 1), Point::new(0, 5)).unwrap();
    let left_vertical = Line::new(Point::new(0, 0), Point::new(4, 0)).unwrap();
    let right_vertical = Line::new(Point::new(0, 4), Point::new(4, 4)).unwrap();
    let offset_vertical = Line::new(Point::new(1, 0), Point::new(5, 0)).unwrap();

    assert!(upper_horizontal.is_horizontal());
    assert!(!upper_horizontal.is_vertical());
    assert!(lower_horizontal.is_horizontal());
    assert!(!lower_horizontal.is_vertical());
    assert!(offset_horizontal.is_horizontal());
    assert!(!offset_horizontal.is_vertical());

    assert!(!left_vertical.is_horizontal());
    assert!(left_vertical.is_vertical());
    assert!(!right_vertical.is_horizontal());
    assert!(right_vertical.is_vertical());
    assert!(!offset_vertical.is_horizontal());
    assert!(offset_vertical.is_vertical());
  }
}
