use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Wire<T: Copy + Default> {
  pub output: Signal<T>,
  input: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Block<T> for Wire<T> {
  fn step(&mut self) {
    self.output.borrow_mut().set(*self.input.borrow().read());
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Wire<T> {
  pub fn new(input: &Signal<T>) -> Self {
    Wire {
      output: new_signal(Default::default()),
      input: Rc::clone(input),
    }
  }
}
