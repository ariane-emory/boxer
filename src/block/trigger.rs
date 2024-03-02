use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RisingTrigger {
  output: Signal<bool>,
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
impl HasSignal<bool> for RisingTrigger {
  fn step(&mut self) {
    if *self.input.borrow().read() && !*self.input.borrow().read() {
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
  output: Signal<bool>,
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
impl HasSignal<bool> for FallingTrigger {
  fn step(&mut self) {
    if !*self.input.borrow().read() && *self.input.borrow().read() {
      self.output.borrow_mut().set(true);
    } else {
      self.output.borrow_mut().set(false);
    }
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}
