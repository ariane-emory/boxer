#![allow(unused_variables)]

#[cfg(test)]
mod tests {
  use squares::geo::*;
  // use squares::geo::Point;
  // use squares::geo::Positional;

  #[test]
  fn point_test() {
    let upper_left = Point::new(0, 0);
    let upper_middle = Point::new(4, 0);
    let upper_right = Point::new(8, 0);

    let left_of_center = Point::new(0, 4);
    let center = Point::new(4, 4);
    let right_of_center = Point::new(8, 4);

    let lower_left = Point::new(0, 8);
    let lower_middle = Point::new(4, 8);
    let lower_right = Point::new(8, 8);

    assert_eq!(center.size(), Size::new(1, 1));
    assert_eq!(center.size().area(), 1);
    assert_eq!(center.size().height, 1);
    assert_eq!(center.size().width, 1);
    assert_eq!(center.left_bound(), 4);
    assert_eq!(center.right_bound(), 4);
    assert_eq!(center.upper_bound(), 4);
    assert_eq!(center.lower_bound(), 4);

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

    assert!(center.is_below(&upper_left));
    assert!(center.is_below(&upper_middle));
    assert!(center.is_below(&upper_right));
    assert!(!center.is_below(&left_of_center));
    assert!(!center.is_below(&right_of_center));
    assert!(!center.is_below(&center));
    assert!(!center.is_below(&lower_left));
    assert!(!center.is_below(&lower_middle));
    assert!(!center.is_below(&lower_right));
  }

  #[test]
  fn line_test() {
    let upper_horizontal = Line::new(0, 0, 4, 0).unwrap();
    let lower_horizontal = Line::new(0, 2, 4, 2).unwrap();
    let offset_horizontal = Line::new(1, 5, 5, 5).unwrap();
    //   012345
    // 0 xxxxx_
    // 1 ______
    // 2 xxxxx_
    // 3 ______
    // 5 ______
    // 5 _xxxxx

    let left_vertical = Line::new(0, 0, 0, 4).unwrap();
    let right_vertical = Line::new(4, 0, 4, 4).unwrap();
    let offset_vertical = Line::new(5, 1, 5, 5).unwrap();
    //   012345
    // 0 x___x_
    // 1 x___xx
    // 2 x___xx
    // 3 x___xx
    // 4 x___xx
    // 5 _____x

    assert!(!upper_horizontal.overlaps(&lower_horizontal));
    assert!(!upper_horizontal.overlaps(&offset_horizontal));
    assert!(upper_horizontal.overlaps(&left_vertical));
    assert!(upper_horizontal.overlaps(&right_vertical));
    assert!(!upper_horizontal.overlaps(&offset_vertical));

    assert!(!left_vertical.overlaps(&right_vertical));
    assert!(!left_vertical.overlaps(&offset_vertical));
    assert!(left_vertical.overlaps(&upper_horizontal));
    assert!(left_vertical.overlaps(&lower_horizontal));
    assert!(!left_vertical.overlaps(&offset_horizontal));

    assert!(upper_horizontal.is_horizontal());
    assert!(lower_horizontal.is_horizontal());
    assert!(offset_horizontal.is_horizontal());
    assert!(!left_vertical.is_horizontal());
    assert!(!right_vertical.is_horizontal());
    assert!(!offset_vertical.is_horizontal());

    assert!(!upper_horizontal.is_vertical());
    assert!(!lower_horizontal.is_vertical());
    assert!(!offset_horizontal.is_vertical());
    assert!(left_vertical.is_vertical());
    assert!(right_vertical.is_vertical());
    assert!(offset_vertical.is_vertical());

    assert_eq!(upper_horizontal.size(), Size::new(1, 5));
    assert_eq!(lower_horizontal.size(), Size::new(1, 5));
    assert_eq!(offset_horizontal.size(), Size::new(1, 5));
    assert_eq!(left_vertical.size(), Size::new(5, 1));
    assert_eq!(right_vertical.size(), Size::new(5, 1));
    assert_eq!(offset_vertical.size(), Size::new(5, 1));

    assert_eq!(upper_horizontal.area(), 5);
    assert_eq!(lower_horizontal.area(), 5);
    assert_eq!(offset_horizontal.area(), 5);
    assert_eq!(left_vertical.area(), 5);
    assert_eq!(right_vertical.area(), 5);
    assert_eq!(offset_vertical.area(), 5);

    assert_eq!(upper_horizontal.height(), 1);
    assert_eq!(lower_horizontal.height(), 1);
    assert_eq!(offset_horizontal.height(), 1);
    assert_eq!(left_vertical.height(), 5);
    assert_eq!(right_vertical.height(), 5);
    assert_eq!(offset_vertical.height(), 5);

    assert_eq!(upper_horizontal.width(), 5);
    assert_eq!(lower_horizontal.width(), 5);
    assert_eq!(offset_horizontal.width(), 5);
    assert_eq!(left_vertical.width(), 1);
    assert_eq!(right_vertical.width(), 1);
    assert_eq!(offset_vertical.width(), 1);

    assert_eq!(upper_horizontal.length(), 5);
    assert_eq!(lower_horizontal.length(), 5);
    assert_eq!(offset_horizontal.length(), 5);
    assert_eq!(left_vertical.length(), 5);
    assert_eq!(right_vertical.length(), 5);
    assert_eq!(offset_vertical.length(), 5);

    assert!(!left_vertical.is_horizontal());
    assert!(!offset_vertical.is_horizontal());
    assert!(!right_vertical.is_horizontal());
    assert!(lower_horizontal.is_horizontal());
    assert!(offset_horizontal.is_horizontal());
    assert!(upper_horizontal.is_horizontal());

    assert!(!lower_horizontal.is_vertical());
    assert!(!offset_horizontal.is_vertical());
    assert!(!upper_horizontal.is_vertical());
    assert!(left_vertical.is_vertical());
    assert!(offset_vertical.is_vertical());
    assert!(right_vertical.is_vertical());

    assert!(upper_horizontal.is_parallel_to(&lower_horizontal));
    assert!(upper_horizontal.is_parallel_to(&offset_horizontal));
    assert!(!upper_horizontal.is_parallel_to(&left_vertical));
    assert!(!upper_horizontal.is_parallel_to(&right_vertical));
    assert!(!upper_horizontal.is_parallel_to(&offset_vertical));

    assert!(left_vertical.is_parallel_to(&right_vertical));
    assert!(left_vertical.is_parallel_to(&offset_vertical));
    assert!(!left_vertical.is_parallel_to(&upper_horizontal));
    assert!(!left_vertical.is_parallel_to(&lower_horizontal));
    assert!(!left_vertical.is_parallel_to(&offset_horizontal));

    assert!(!upper_horizontal.is_perpendicular_to(&lower_horizontal));
    assert!(!upper_horizontal.is_perpendicular_to(&offset_horizontal));
    assert!(upper_horizontal.is_perpendicular_to(&left_vertical));
    assert!(upper_horizontal.is_perpendicular_to(&right_vertical));
    assert!(upper_horizontal.is_perpendicular_to(&offset_vertical));

    assert!(!left_vertical.is_perpendicular_to(&right_vertical));
    assert!(!left_vertical.is_perpendicular_to(&offset_vertical));
    assert!(left_vertical.is_perpendicular_to(&upper_horizontal));
    assert!(left_vertical.is_perpendicular_to(&lower_horizontal));
    assert!(left_vertical.is_perpendicular_to(&offset_horizontal));

    assert!(upper_horizontal.is_horizontally_coaligned_with(&lower_horizontal));
    assert!(!upper_horizontal.is_horizontally_coaligned_with(&offset_horizontal));
    assert!(!upper_horizontal.is_vertically_coaligned_with(&left_vertical));
    assert!(!upper_horizontal.is_vertically_coaligned_with(&right_vertical));
    assert!(!upper_horizontal.is_vertically_coaligned_with(&offset_vertical));

    assert!(left_vertical.is_vertically_coaligned_with(&right_vertical));
    assert!(!left_vertical.is_vertically_coaligned_with(&offset_vertical));
    assert!(!left_vertical.is_horizontally_coaligned_with(&upper_horizontal));
    assert!(!left_vertical.is_horizontally_coaligned_with(&lower_horizontal));
    assert!(!left_vertical.is_horizontally_coaligned_with(&offset_horizontal));

    assert!(upper_horizontal
      .is_coaligned_with(&lower_horizontal)
      .is_some());
    assert!(upper_horizontal
      .is_coaligned_with(&offset_horizontal)
      .is_none());
    assert!(upper_horizontal.is_coaligned_with(&left_vertical).is_none());
    assert!(upper_horizontal
      .is_coaligned_with(&right_vertical)
      .is_none());
    assert!(upper_horizontal
      .is_coaligned_with(&offset_vertical)
      .is_none());

    assert!(left_vertical.is_coaligned_with(&right_vertical).is_some());
    assert!(left_vertical.is_coaligned_with(&offset_vertical).is_none());
    assert!(left_vertical.is_coaligned_with(&upper_horizontal).is_none());
    assert!(left_vertical.is_coaligned_with(&lower_horizontal).is_none());
    assert!(left_vertical
      .is_coaligned_with(&offset_horizontal)
      .is_none());
  }
}
