use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RisingTrigger {
  pub output: Signal<bool>,
  input: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RisingTrigger {
  pub fn new(input: &Signal<bool>) -> Self {
    RisingTrigger {
      output: new_signal(false),
      input: Rc::clone(input),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for RisingTrigger {
  fn step(&mut self) {
    let last_state = *self.input.borrow().read();
    let input = *self.input.borrow().read();

    if input && !last_state {
      self.output.borrow_mut().set(true);
    } else {
      self.output.borrow_mut().set(false);
    }
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct FallingTrigger {
  pub output: Signal<bool>,
  input: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FallingTrigger {
  pub fn new(input: &Signal<bool>) -> Self {
    FallingTrigger {
      output: new_signal(false),
      input: Rc::clone(input),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for FallingTrigger {
  fn step(&mut self) {
    let last_state = *self.input.borrow().read();
    let input = *self.input.borrow().read();

    if !input && last_state {
      self.output.borrow_mut().set(true);
    } else {
      self.output.borrow_mut().set(false);
    }
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}
