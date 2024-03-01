use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Counter {
  pub output: Signal<usize>,
  pub at_max: Signal<bool>,
  input: Signal<bool>,
  reset: Signal<bool>,
  max: Signal<usize>,
  last_input_state: bool,
  last_reset_state: bool,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Counter {
  pub fn new(input: &Signal<bool>, reset: &Signal<bool>, max: &Signal<usize>) -> Self {
    Counter {
      output: new_signal(0),
      at_max: new_signal(false),
      input: Rc::clone(input),
      reset: Rc::clone(reset),
      max: Rc::clone(max),
      last_input_state: false,
      last_reset_state: false,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<usize> for Counter {
  fn step(&mut self) {
    let output_val = *self.output.borrow().read();
    let max_val = *self.max.borrow().read();
    let input_val = *self.input.borrow().read();
    let reset_val = *self.reset.borrow().read();
    let at_max_val = *self.at_max.borrow().read();
    let last_input_state_val = self.last_input_state;
    let last_reset_state_val = self.last_reset_state;
    let input_rose = input_val && !last_input_state_val;
    let reset_rose = reset_val && !last_reset_state_val;

    if reset_rose {
      self.output.borrow_mut().set(0);
      self.at_max.borrow_mut().set(false);
    } else if at_max_val {
      return;
    }
    if input_rose {
      self.output.borrow_mut().set(output_val + 1);

      if output_val + 1 == max_val {
        self.at_max.borrow_mut().set(true);
      }
    }
  }

  fn output(&self) -> &Signal<usize> {
    &self.output
  }
}
