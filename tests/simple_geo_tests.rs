////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
  use boxer::simple_geo::*;

  //////////////////////////////////////////////////////////////////////////////////////////////////
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
    assert_eq!(center.size().height(), 1);
    assert_eq!(center.size().width(), 1);
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

  //////////////////////////////////////////////////////////////////////////////////////////////////
  #[test]
  fn point_distance() {
    let upper_left = Point::new(0, 0);
    let upper_middle = Point::new(0, 1);
    let upper_right = Point::new(0, 2);

    let left_of_center = Point::new(1, 0);
    let center = Point::new(1, 1);
    let right_of_center = Point::new(1, 2);

    let lower_left = Point::new(2, 0);
    let lower_middle = Point::new(2, 1);
    let lower_right = Point::new(2, 2);

    assert_eq!(upper_left.distance(&upper_left), 0);

    assert_eq!(upper_left.distance(&upper_middle), 1);
    assert_eq!(upper_middle.distance(&upper_left), 1);

    assert_eq!(upper_left.distance(&upper_right), 2);
    assert_eq!(upper_right.distance(&upper_left), 2);

    assert_eq!(upper_left.distance(&left_of_center), 1);
    assert_eq!(left_of_center.distance(&upper_left), 1);

    assert_eq!(upper_left.distance(&center), 2);
    assert_eq!(center.distance(&upper_left), 2);

    assert_eq!(upper_left.distance(&right_of_center), 3);
    assert_eq!(right_of_center.distance(&upper_left), 3);

    assert_eq!(upper_left.distance(&lower_left), 2);
    assert_eq!(lower_left.distance(&upper_left), 2);

    assert_eq!(upper_left.distance(&lower_middle), 3);
    assert_eq!(lower_middle.distance(&upper_left), 3);

    assert_eq!(upper_left.distance(&lower_right), 4);
    assert_eq!(lower_right.distance(&upper_left), 4);
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  #[test]
  fn line_test() {
    //assert!(Line::new(Point::new(0, 0), Point::new(0, 0)).is_err());
    assert!(Line::new(Point::new(0, 0), Point::new(0, 0)).is_ok());
    assert!(Line::new(Point::new(0, 0), Point::new(1, 0)).is_ok());
    assert!(Line::new(Point::new(0, 0), Point::new(0, 1)).is_ok());
    assert!(Line::new(Point::new(0, 0), Point::new(1, 1)).is_err());
    assert!(Line::new(Point::new(0, 0), Point::new(2, 1)).is_err());
    assert!(Line::new(Point::new(0, 0), Point::new(1, 2)).is_err());

    let upper_horizontal =
      Line::new(Point::new(0, 0), Point::new(0, 4)).unwrap();
    let lower_horizontal =
      Line::new(Point::new(2, 0), Point::new(2, 4)).unwrap();
    let offset_horizontal =
      Line::new(Point::new(5, 1), Point::new(5, 5)).unwrap();
    //   012345
    // 0 xxxxx_
    // 1 ______
    // 2 xxxxx_
    // 3 ______
    // 5 ______
    // 5 _xxxxx

    let left_vertical = Line::new(Point::new(0, 0), Point::new(4, 0)).unwrap();
    let right_vertical = Line::new(Point::new(0, 4), Point::new(4, 4)).unwrap();
    let offset_vertical =
      Line::new(Point::new(1, 5), Point::new(5, 5)).unwrap();
    //   012345
    // 0 x___x_
    // 1 x___xx
    // 2 x___xx
    // 3 x___xx
    // 4 x___xx
    // 5 _____x

    assert!(upper_horizontal.is_connected_to(&left_vertical));
    assert!(left_vertical.is_connected_to(&upper_horizontal));

    assert!(upper_horizontal.is_connected_to(&right_vertical));
    assert!(right_vertical.is_connected_to(&upper_horizontal));

    assert!(!upper_horizontal.is_connected_to(&offset_vertical));
    assert!(!offset_vertical.is_connected_to(&upper_horizontal));

    assert!(!upper_horizontal.is_connected_to(&lower_horizontal));
    assert!(!lower_horizontal.is_connected_to(&upper_horizontal));

    assert!(!upper_horizontal.is_connected_to(&offset_horizontal));
    assert!(!offset_horizontal.is_connected_to(&upper_horizontal));

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

    assert_eq!(upper_horizontal.len(), 5);
    assert_eq!(lower_horizontal.len(), 5);
    assert_eq!(offset_horizontal.len(), 5);
    assert_eq!(left_vertical.len(), 5);
    assert_eq!(right_vertical.len(), 5);
    assert_eq!(offset_vertical.len(), 5);

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
    assert!(
      !upper_horizontal.is_horizontally_coaligned_with(&offset_horizontal)
    );
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

  //////////////////////////////////////////////////////////////////////////////////////////////////
  #[test]
  fn rectangle_test() {
    assert!(Rectangle::new(Point::new(0, 0), Point::new(0, 0)).is_err());
    assert!(Rectangle::new(Point::new(0, 0), Point::new(1, 0)).is_err());
    assert!(Rectangle::new(Point::new(0, 0), Point::new(0, 1)).is_err());
    assert!(Rectangle::new(Point::new(0, 0), Point::new(1, 1)).is_err());
    assert!(Rectangle::new(Point::new(0, 0), Point::new(2, 1)).is_err());
    assert!(Rectangle::new(Point::new(0, 0), Point::new(1, 2)).is_err());
    assert!(Rectangle::new(Point::new(0, 0), Point::new(2, 2)).is_ok());

    let rect = Rectangle::new(Point::new(10, 0), Point::new(15, 5)).unwrap();
    let overlapping_rect =
      Rectangle::new(Point::new(13, 3), Point::new(18, 8)).unwrap();
    let nonoverlapping_rect =
      Rectangle::new(Point::new(16, 16), Point::new(20, 20)).unwrap();

    assert!(rect.width() == 6);
    assert!(rect.height() == 6);
    assert!(rect.area() == 36);

    assert!(rect.overlaps(&overlapping_rect));
    assert!(!rect.overlaps(&nonoverlapping_rect));

    assert!(overlapping_rect.overlaps(&rect));
    assert!(!nonoverlapping_rect.overlaps(&rect));

    let rect1 = Rectangle::new(Point::new(10, 10), Point::new(20, 20)).unwrap();

    assert_eq!(
      rect1.contained_rectangle().unwrap(),
      Rectangle::new(Point::new(11, 11), Point::new(19, 19)).unwrap()
    );

    assert_eq!(
      Rectangle::new(Point::new(0, 0), Point::new(2, 2))
        .unwrap()
        .contained_rectangle()
        .unwrap(),
      Rectangle {
        top_left: Point::new(1, 1),
        bottom_right: Point::new(1, 1)
      }
    );

    assert_eq!(
      Rectangle::new(Point::new(0, 0), Point::new(2, 2))
        .unwrap()
        .contained_rectangle()
        .unwrap(),
      Rectangle {
        top_left: Point::new(1, 1),
        bottom_right: Point::new(1, 1)
      }
    );
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  #[test]
  #[should_panic]
  fn rectangle_panic_test_1() {
    Rectangle::new(Point::new(0, 0), Point::new(1, 1)).unwrap();
  }

  // This one is probably obsolete:
  // ///////////////////////////////////////////////////////////////////////////
  // /////////////////////// #[test]
  // #[should_panic]
  // fn rectangle_panic_test_2() {
  //   Rectangle {
  //     top_left: Point::new(1, 1),
  //     bottom_right: Point::new(1, 1),
  //   }
  //   .contained_rectangle()
  //   .unwrap();
  // }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  #[test]
  #[should_panic]
  fn rectangle_panic_test_3() {
    Rectangle {
      top_left: Point::new(1, 1),
      bottom_right: Point::new(5, 1),
    }
    .contained_rectangle()
    .unwrap();
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  #[test]
  fn line_touches_rectangle_test() {
    let rect1 = Rectangle::new(Point::new(10, 10), Point::new(20, 20)).unwrap();
    let rect2 = Rectangle::new(Point::new(10, 30), Point::new(20, 40)).unwrap();

    // A horizontal line touching the right edge of rect1 and the left edge of
    // rect2:
    let line1 = Line::new(Point::new(15, 20), Point::new(15, 30)).unwrap();
    assert!(line1.touches(&rect1));
    assert!(line1.touches(&rect2));
    assert!(line1.overlaps(&rect1));
    assert!(line1.overlaps(&rect2));

    // A horizontal line touching the right edge of rect1 that doesn't quite
    // reach the left edge of rect2:
    let line2 = Line::new(Point::new(15, 20), Point::new(15, 29)).unwrap();
    assert!(line2.touches(&rect1));
    assert!(!line2.touches(&rect2));
    assert!(line2.overlaps(&rect1));
    assert!(!line2.overlaps(&rect2));

    // A horizontal line touching the left edge of rect2 that doesn't quite
    // reach the right edge of rect2:
    let line3 = Line::new(Point::new(15, 21), Point::new(15, 30)).unwrap();
    assert!(!line3.touches(&rect1));
    assert!(line3.touches(&rect2));
    assert!(!line3.overlaps(&rect1));
    assert!(line3.overlaps(&rect2));

    // A horizontal line between rect1 and the left edge of rect2 that overlaps
    // with rect1:
    let line4 = Line::new(Point::new(15, 19), Point::new(15, 30)).unwrap();
    assert!(!line4.touches(&rect1));
    assert!(line4.touches(&rect2));
    assert!(line4.overlaps(&rect1));
    assert!(line4.overlaps(&rect2));

    // A horizontal line between the right edge of rect1 and the left edge of
    // rect2 that overlaps with rect2:
    let line5 = Line::new(Point::new(15, 20), Point::new(15, 31)).unwrap();
    assert!(line5.touches(&rect1));
    assert!(!line5.touches(&rect2));
    assert!(line5.overlaps(&rect1));
    assert!(line5.overlaps(&rect2));

    // A horizontal line between the right edge of rect1 and the left edge of
    // rect2 that overlaps with both:
    let line6 = Line::new(Point::new(15, 19), Point::new(15, 41)).unwrap();
    assert!(!line6.touches(&rect1));
    assert!(!line6.touches(&rect2));
    assert!(line6.overlaps(&rect1));
    assert!(line6.overlaps(&rect2));

    let lower_rect =
      Rectangle::new(Point::new(30, 10), Point::new(40, 20)).unwrap();

    // A vertical line touching the bottom edge of rect1 and the top edge of
    // lower_rect (but not overlapping with either):
    let line7 = Line::new(Point::new(20, 15), Point::new(30, 15)).unwrap();
    assert!(line7.touches(&rect1));
    assert!(line7.touches(&lower_rect));
    assert!(line7.overlaps(&rect1));
    assert!(line7.overlaps(&lower_rect));

    // A vertical line touching the bottom edge of rect1 that doesn't quite
    // reach the top edge of lower_rect:
    let line8 = Line::new(Point::new(20, 15), Point::new(29, 15)).unwrap();
    assert!(line8.touches(&rect1));
    assert!(!line8.touches(&lower_rect));
    assert!(line8.overlaps(&rect1));
    assert!(!line8.overlaps(&lower_rect));

    // A vertical line touching the top edge of lower_rect that doesn't quite
    // reach the bottom edge of rect1:
    let line9 = Line::new(Point::new(21, 15), Point::new(30, 15)).unwrap();
    assert!(!line9.touches(&rect1));
    assert!(line9.touches(&lower_rect));
    assert!(!line9.overlaps(&rect1));
    assert!(line9.overlaps(&lower_rect));

    // A vertical line between rect1 and the top edge of lower_rect that
    // overlaps with rect1:
    let line10 = Line::new(Point::new(19, 15), Point::new(30, 15)).unwrap();
    assert!(!line10.touches(&rect1));
    assert!(line10.touches(&lower_rect));
    assert!(line10.overlaps(&rect1));
    assert!(line10.overlaps(&lower_rect));

    // A vertical line between the bottom edge of rect1 and the top edge of
    // lower_rect that overlaps with lower_rect:
    let line11 = Line::new(Point::new(20, 15), Point::new(31, 15)).unwrap();
    assert!(line11.touches(&rect1));
    assert!(!line11.touches(&lower_rect));
    assert!(line11.overlaps(&rect1));
    assert!(line11.overlaps(&lower_rect));

    // A horizontal line that isn't properly touching rect1 and rect2 because
    // it strikes their corners:
    let line12 = Line::new(Point::new(10, 10), Point::new(10, 30)).unwrap();
    assert!(!line12.touches(&rect1));
    assert!(line12.overlaps(&rect1));
    assert!(!line12.touches(&rect2));
    assert!(line12.overlaps(&rect2));

    // A vertical line that isn't properly touching rect1 and lower_rect
    // because it strikes their corners:
    let line13 = Line::new(Point::new(10, 10), Point::new(30, 10)).unwrap();
    assert!(!line13.touches(&rect1));
    assert!(line13.overlaps(&rect1));
    assert!(!line13.touches(&lower_rect));
    assert!(line13.overlaps(&lower_rect));
  }

  //////////////////////////////////////////////////////////////////////////////////////////////////
  #[test]
  fn find_rectangle_test() {
    //                   1111111111111111
    //   0123457890abcdef0123456789abcdef
    // 0 xxxxxxx
    // 1 x     x
    // 3 x     x
    // 4 x  xxxxxxx
    // 5 x  x  x  x
    // 6 x  x  x  x
    // 7 xxxxxxx  x
    // 8    x     x
    // 9    xxxxxxx

    let lines = vec![
      Line::new(Point::new(0, 0), Point::new(0, 7)).unwrap(),
      Line::new(Point::new(4, 3), Point::new(4, 16)).unwrap(),
      Line::new(Point::new(7, 0), Point::new(7, 7)).unwrap(),
      Line::new(Point::new(9, 3), Point::new(9, 16)).unwrap(),
      Line::new(Point::new(0, 0), Point::new(7, 0)).unwrap(),
      Line::new(Point::new(0, 7), Point::new(7, 7)).unwrap(),
      Line::new(Point::new(4, 3), Point::new(9, 3)).unwrap(),
      Line::new(Point::new(4, 16), Point::new(9, 16)).unwrap(),
    ];

    let (rects, leftover_lines) = find_rectangles(lines, false);

    println!("");
    for rect in &rects {
      println!("Discovered Rectangle: {:?}", rect);
    }

    println!("");
    for line in &leftover_lines {
      println!("Leftover Line: {:?}", line);
    }

    assert!(rects.len() == 2);
    assert!(leftover_lines.len() == 0);
    assert_eq!(
      rects[0],
      Rectangle::new(Point::new(4, 3), Point::new(9, 16)).unwrap()
    );
    assert_eq!(
      rects[1],
      Rectangle {
        top_left: Point::new(0, 0),
        bottom_right: Point::new(7, 7)
      }
    );
  }
}
//////////////////////////////////////////////////////////////////////////////////////////////////////
