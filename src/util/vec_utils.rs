/////////////////////////////////////////////////////////////////////////////////
pub fn vec_remove<T: PartialEq + std::fmt::Debug>(vec: &mut Vec<T>, value: &T) {
  let pos = vec.iter().position(|v| v == value);

  if let Some(pos) = pos {
    vec.remove(pos);
  }
  else {
    panic!("Value {:?} not found in vec.", value);
  }
}

/////////////////////////////////////////////////////////////////////////////////
pub fn vec_sorted_insert<T: Ord>(vec: &mut Vec<T>, value: T) {
  match vec.binary_search(&value) {
    Ok(pos) => vec.insert(pos, value),
    Err(pos) => vec.insert(pos, value),
  }
}
