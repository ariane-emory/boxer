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
    let input = *self.input.borrow().read();
    let max = *self.max.borrow().read();
    let last_state = self.last_state;

    if input && !last_state {
      let mut output = self.output.borrow_mut();
      let mut at_max = self.at_max.borrow_mut();

      if *output.read() < max {
        output.set(*self.output.borrow().read() + 1);
        at_max.set(*self.output.borrow().read() == max);
      }
    }

    self.last_state = input;
  }

  fn output(&self) -> &Signal<usize> {
    &self.output
  }
}
