use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Jump<T: Copy + Default> {
  pub output: Signal<T>,
  pub input: Option<Signal<T>>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Jump<T> {
  //  pub fn new(input: Option<Signal<T>>) -> Self {
  pub fn new() -> Self {
    // if let Some(input) = input {
    //   Jump {
    //     output: new_signal(Default::default()),
    //     input: Some(Rc::clone(&input)),
    //   }
    // } else {
    Jump {
      output: new_signal(Default::default()),
      input: None,
    }
    //}
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Block<T> for Jump<T> {
  fn step(&mut self) {
    if let Some(input) = &self.input {
      self.output.borrow_mut().set(*input.borrow().read());
    }
    // else {
    //   self.output.borrow_mut().set(Default::default());
    // }
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}
