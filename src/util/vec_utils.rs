/////////////////////////////////////////////////////////////////////////////////
pub fn vec_sorted_insert<T: Ord>(vec: &mut Vec<T>, value: T) {
  match vec.binary_search(&value) {
    Ok(pos) => vec.insert(pos, value),
    Err(pos) => vec.insert(pos, value),
  }
}

// /////////////////////////////////////////////////////////////////////////////
// /// pub trait VecRemove<T>
// where
//   T: PartialEq,
// {
//   fn remove(&mut self, value: &T);
// }

// impl<T: PartialEq> VecRemove<T> for Vec<T> {
//   fn remove(&mut self, value: &T) {
//     if let Some(pos) = self.iter().position(|x| x == value) {
//       self.remove(pos);
//     }
//   }
// }

////////////////////////////////////////////////////////////////////////////////
pub trait VecRemoveq<T>
where
  T: PartialEq,
{
  fn removeq(&mut self, value: &T);
}

impl<T: PartialEq> VecRemoveq<T> for Vec<T> {
  fn removeq(&mut self, value: &T) {
    if let Some(pos) = self.iter().position(|x| x == value) {
      self.remove(pos);
    }
  }
}
