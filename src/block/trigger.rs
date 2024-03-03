use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct RisingTrigger {
  output: OutputSignalRef<bool>,
  input: OutputSignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl RisingTrigger {
  pub fn new(input: &OutputSignalRef<bool>) -> Self {
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
  fn output(&self) -> &OutputSignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct FallingTrigger {
  output: OutputSignalRef<bool>,
  input: OutputSignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl FallingTrigger {
  pub fn new(input: &OutputSignalRef<bool>) -> Self {
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
  fn output(&self) -> &OutputSignalRef<bool> {
    &self.output
  }
}
