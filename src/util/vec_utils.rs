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
    if let Some(pos) = self.iter().position(|x| x == value) {
      Some(self.remove(pos))
    }
    else {
      None
    }
  }
}
