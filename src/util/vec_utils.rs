////////////////////////////////////////////////////////////////////////////////
pub trait SortedInsert<T>
where
  T: Ord,
{
  fn sorted_insert(&mut self, value: T);
}

impl<T: Ord> SortedInsert<T> for Vec<T> {
  fn sorted_insert(&mut self, value: T) {
    match self.binary_search(&value) {
      Ok(pos) => self.insert(pos, value),
      Err(pos) => self.insert(pos, value),
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
pub trait Removeql<T>
where
  T: PartialEq,
{
  fn removeql(&mut self, value: &T) -> Option<T>;
}

impl<T: PartialEq> Removeql<T> for Vec<T> {
  fn removeql(&mut self, value: &T) -> Option<T> {
    self
      .iter()
      .position(|x| x == value)
      .map(|pos| self.remove(pos))
  }
}

////////////////////////////////////////////////////////////////////////////////
pub trait RemoveIf<T> {
  fn remove_if<F>(&mut self, predicate: F) -> Option<T>
  where
    F: Fn(&T) -> bool;
}

impl<T> RemoveIf<T> for Vec<T> {
  fn remove_if<F>(&mut self, predicate: F) -> Option<T>
  where
    F: Fn(&T) -> bool,
  {
    self.iter().position(predicate).map(|pos| self.remove(pos))
  }
}
