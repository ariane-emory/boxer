use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Select<T: Copy> {
  pub output: Signal<T>,
  which: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Select<T> {
  pub fn new(which: &Signal<bool>, left: &Signal<T>, right: &Signal<T>) -> Self {
    Select {
      output: new_signal(*left.borrow().read()),
      which: Rc::clone(which),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Block<T> for Select<T> {
  fn step(&mut self) {
    if *self.which.borrow().read() {
      self.output.borrow_mut().set(*self.left.borrow().read());
    } else {
      self.output.borrow_mut().set(*self.right.borrow().read());
    }
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}
