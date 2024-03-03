use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct SRLatch {
  output: SignalRef<bool>,
  set: SignalRef<bool>,
  reset: SignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl SRLatch {
  pub fn new(
    set: &SignalRef<bool>,
    reset: &SignalRef<bool>,
  ) -> Self {
    SRLatch {
      output: new_signal_ref(false),
      set: Rc::clone(set),
      reset: Rc::clone(reset),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for SRLatch {
  fn step(&mut self) {
    if self.reset.read() {
      self.output.set(false);
    } else if self.set.read() {
      self.output.set(true);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for SRLatch {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct RSLatch {
  output: SignalRef<bool>,
  set: SignalRef<bool>,
  reset: SignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl RSLatch {
  pub fn new(
    set: &SignalRef<bool>,
    reset: &SignalRef<bool>,
  ) -> Self {
    RSLatch {
      output: new_signal_ref(false),
      set: Rc::clone(set),
      reset: Rc::clone(reset),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for RSLatch {
  fn step(&mut self) {
    if self.reset.read() {
      self.output.set(false);
    } else if self.set.read() {
      self.output.set(true);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for RSLatch {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct JKFlipFlop {
  output: SignalRef<bool>,
  j_input: SignalRef<bool>,
  k_input: SignalRef<bool>,
  clock: SignalRef<bool>,
  last_clock: bool,
}
////////////////////////////////////////////////////////////////////////////////
impl JKFlipFlop {
  pub fn new(
    j_input: &SignalRef<bool>,
    k_input: &SignalRef<bool>,
    clock: &SignalRef<bool>,
  ) -> Self {
    JKFlipFlop {
      output: new_signal_ref(false),
      j_input: Rc::clone(j_input),
      k_input: Rc::clone(k_input),
      clock: Rc::clone(clock),
      last_clock: false,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for JKFlipFlop {
  fn step(&mut self) {
    if self.last_clock && !self.clock.read() {
      if self.j_input.read() && self.k_input.read() {
        self.output.set(!self.output.read());
      } else if self.j_input.read() {
        self.output.set(true);
      } else if self.k_input.read() {
        self.output.set(false);
      }
    }
    self.last_clock = self.clock.read();
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for JKFlipFlop {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct DFlipFlop {
  output: SignalRef<bool>,
  input: SignalRef<bool>,
  clock: SignalRef<bool>,
  last_clock: bool,
}
////////////////////////////////////////////////////////////////////////////////
impl DFlipFlop {
  pub fn new(
    input: &SignalRef<bool>,
    clock: &SignalRef<bool>,
  ) -> Self {
    DFlipFlop {
      output: new_signal_ref(false),
      input: Rc::clone(input),
      clock: Rc::clone(clock),
      last_clock: false,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for DFlipFlop {
  fn step(&mut self) {
    if self.last_clock && !self.clock.read() {
      self.output.set(self.input.read());
    }
    self.last_clock = self.clock.read();
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for DFlipFlop {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct TFlipFlop {
  output: SignalRef<bool>,
  input: SignalRef<bool>,
  clock: SignalRef<bool>,
  last_clock: bool,
}
////////////////////////////////////////////////////////////////////////////////
impl TFlipFlop {
  pub fn new(
    input: &SignalRef<bool>,
    clock: &SignalRef<bool>,
  ) -> Self {
    TFlipFlop {
      output: new_signal_ref(false),
      input: Rc::clone(input),
      clock: Rc::clone(clock),
      last_clock: false,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for TFlipFlop {
  fn step(&mut self) {
    if self.last_clock && !self.clock.read() {
      if self.input.read() {
        self.output.set(!self.output.read());
      }
    }
    self.last_clock = self.clock.read();
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for TFlipFlop {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}
