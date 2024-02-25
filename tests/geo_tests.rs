#[cfg(test)]
mod tests {
  #[test]
  fn point_test() {
    let point = squares::geo::Point { line: 0, col: 0 };
    assert_eq!(0, point.line);
  }
}
