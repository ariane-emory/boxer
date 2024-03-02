use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct Feedback<T: Copy + Default> {
  output: Signal<T>,
  input: Option<Signal<T>>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Feedback<T> {
  pub fn new() -> Self {
    Feedback {
      output: new_signal(Default::default()),
      input: None,
    }
    //}
  }

  pub fn set_input(
    &mut self,
    input: &Signal<T>,
  ) {
    self.input = Some(input.clone());
    self.step();
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Steppable for Feedback<T> {
  fn step(&mut self) {
    if let Some(input) = &self.input {
      self.output.borrow_mut().set(*input.borrow().read());
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> HasOutputSignal<T> for Feedback<T> {
  fn output(&self) -> &Signal<T> {
    &self.output
  }
}
