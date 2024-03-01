use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Counter {
  pub output: Signal<usize>,
  pub at_max: Signal<bool>,
  input: Signal<bool>,
  max: Signal<usize>,
  last_state: bool,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Counter {
  pub fn new(input: &Signal<bool>, max: &Signal<usize>) -> Self {
    Counter {
      output: new_signal(0),
      at_max: new_signal(false),
      input: Rc::clone(input),
      max: Rc::clone(max),
      last_state: false,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<usize> for Counter {
  fn step(&mut self) {
    let output_val = *self.output.borrow().read();
    let max_val = *self.max.borrow().read();
    let input_val = *self.input.borrow().read();
    let at_max_val = *self.at_max.borrow().read();
    let last_state_val = self.last_state;

    if at_max_val {
      return;
    }
  }

  fn output(&self) -> &Signal<usize> {
    &self.output
  }
}
