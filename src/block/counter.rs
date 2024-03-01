use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct UpCounter {
  output: Signal<usize>,
  at_max: Signal<bool>,
  input: Signal<bool>,
  reset: Signal<bool>,
  max: Signal<usize>,
  last_input_state: bool,
  last_reset_state: bool,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UpCounter {
  pub fn new(input: &impl Block<bool>, reset: &impl Block<bool>, max: &impl Block<usize>) -> Self {
    UpCounter {
      output: new_signal(0),
      at_max: new_signal(false),
      input: Rc::clone(input.output()),
      reset: Rc::clone(reset.output()),
      max: Rc::clone(max.output()),
      last_input_state: false,
      last_reset_state: false,
    }
  }

  pub fn at_max(&self) -> &Signal<bool> {
    &self.at_max
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<usize> for UpCounter {
  fn step(&mut self) {
    let output_val = *self.output.borrow().read();
    let max_val = *self.max.borrow().read();
    let input_val = *self.input.borrow().read();
    let reset_val = *self.reset.borrow().read();
    let at_max = max_val == output_val;
    let last_input_state_val = self.last_input_state;
    let last_reset_state_val = self.last_reset_state;
    let input_rose = input_val && !last_input_state_val;
    let reset_rose = reset_val && !last_reset_state_val;

    self.last_input_state = input_val;
    self.last_reset_state = reset_val;

    self.at_max.borrow_mut().set(at_max);

    if reset_rose {
      // println!("Reset rose..");
      self.output.borrow_mut().set(0);
      self.at_max.borrow_mut().set(false);
    } else if at_max {
      // println!("At max!");
      return;
    } else if input_rose {
      // println!("  Input rose..");
      self.output.borrow_mut().set(output_val + 1);
    } else {
      // println!("  Nothing interesting happened..");
    }
  }

  fn output(&self) -> &Signal<usize> {
    &self.output
  }
}
