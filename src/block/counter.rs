use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct UpCounter {
  output: OutputSignalRef<usize>,
  at_max: OutputSignalRef<bool>,
  input: OutputSignalRef<bool>,
  reset: OutputSignalRef<bool>,
  max: OutputSignalRef<usize>,
  last_input_state: bool,
  last_reset_state: bool,
}
////////////////////////////////////////////////////////////////////////////////
impl UpCounter {
  pub fn new(
    input: &OutputSignalRef<bool>,
    reset: &OutputSignalRef<bool>,
    max: &OutputSignalRef<usize>,
  ) -> Self {
    UpCounter {
      output: new_signal(0),
      at_max: new_signal(false),
      input: Rc::clone(input),
      reset: Rc::clone(reset),
      max: Rc::clone(max),
      last_input_state: false,
      last_reset_state: false,
    }
  }

  pub fn at_max(&self) -> &OutputSignalRef<bool> {
    &self.at_max
  }

  pub fn at_max_value(&self) -> bool {
    self.at_max.read()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for UpCounter {
  fn step(&mut self) {
    let output_val = self.output.read();
    let max_val = self.max.read();
    let input_val = self.input.read();
    let reset_val = self.reset.read();
    let at_max = max_val == output_val;
    let last_input_state_val = self.last_input_state;
    let last_reset_state_val = self.last_reset_state;
    let input_rose = input_val && !last_input_state_val;
    let reset_rose = reset_val && !last_reset_state_val;

    self.last_input_state = input_val;
    self.last_reset_state = reset_val;

    self.at_max.set(at_max);

    if reset_rose {
      // println!("Reset rose..");
      self.output.set(0);
      self.at_max.set(false);
    } else if at_max {
      // println!("At max!");
      return;
    } else if input_rose {
      // println!("  Input rose..");
      self.output.set(output_val + 1);
    } else {
      // println!("  Nothing interesting happened..");
    }
  }
}
///////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<usize> for UpCounter {
  fn output(&self) -> &OutputSignalRef<usize> {
    &self.output
  }
}
