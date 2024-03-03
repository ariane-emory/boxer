use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct RisingTrigger {
  output: Signal<bool>,
  input: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl RisingTrigger {
  pub fn new(input: &Signal<bool>) -> Self {
    RisingTrigger {
      output: new_signal(false),
      input: Rc::clone(input),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for RisingTrigger {
  fn step(&mut self) {
    if !self.input.read() && self.input.read() {
      self.output.set(true);
    } else {
      self.output.set(false);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for RisingTrigger {
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct FallingTrigger {
  output: Signal<bool>,
  input: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl FallingTrigger {
  pub fn new(input: &Signal<bool>) -> Self {
    FallingTrigger {
      output: new_signal(false),
      input: Rc::clone(input),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for FallingTrigger {
  fn step(&mut self) {
    if !self.input.read() && self.input.read() {
      self.output.set(true);
    } else {
      self.output.set(false);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for FallingTrigger {
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}
