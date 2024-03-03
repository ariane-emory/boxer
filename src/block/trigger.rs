use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct RisingTrigger {
  output: SignalRef<bool>,
  input: SignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl RisingTrigger {
  pub fn new(input: &SignalRef<bool>) -> Self {
    RisingTrigger {
      output: new_signal_ref(false),
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
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct FallingTrigger {
  output: SignalRef<bool>,
  input: SignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl FallingTrigger {
  pub fn new(input: &SignalRef<bool>) -> Self {
    FallingTrigger {
      output: new_signal_ref(false),
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
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}
