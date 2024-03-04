////////////////////////////////////////////////////////////////////////////////
pub trait VecSortedInsert<T>
where
  T: Ord,
{
  fn sorted_insert(&mut self, value: T);
}

impl<T: Ord> VecSortedInsert<T> for Vec<T> {
  fn sorted_insert(&mut self, value: T) {
    match self.binary_search(&value) {
      Ok(pos) => self.insert(pos, value),
      Err(pos) => self.insert(pos, value),
    }
  }
}

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
