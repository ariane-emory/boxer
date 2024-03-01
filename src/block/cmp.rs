use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct GreaterThan<T: std::cmp::PartialOrd + Copy> {
  pub output: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> GreaterThan<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    GreaterThan {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> Block<bool> for GreaterThan<T> {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() > *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LessThan<T: std::cmp::PartialOrd + Copy> {
  pub output: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> LessThan<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    LessThan {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> Block<bool> for LessThan<T> {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() < *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}
