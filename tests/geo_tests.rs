#![allow(unused_variables)]

use squares::geo::*;

#[cfg(test)]
mod tests {
  #[test]
  fn point_test() {
    let above = squares::geo::Point::new(0, 0);
    let center = squares::geo::Point::new(4, 4);

    assert_eq!(4, center.line);
  }
}
