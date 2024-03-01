use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Feedback<T: Copy + Default> {
  output: Signal<T>,
  input: Option<Signal<T>>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Feedback<T> {
  //  pub fn new(input: Option<Signal<T>>) -> Self {
  pub fn new() -> Self {
    // if let Some(input) = input {
    //   Feedback {
    //     output: new_signal(Default::default()),
    //     input: Some(Rc::clone(&input)),
    //   }
    // } else {
    Feedback {
      output: new_signal(Default::default()),
      input: None,
    }
    //}
  }

  pub fn set_input(&mut self, input: &Signal<T>) {
    self.input = Some(input.clone());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Block<T> for Feedback<T> {
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
