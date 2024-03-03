use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct SRLatch {
  output: Signal<bool>,
  set: Signal<bool>,
  reset: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl SRLatch {
  pub fn new(
    set: &Signal<bool>,
    reset: &Signal<bool>,
  ) -> Self {
    SRLatch {
      output: new_signal(false),
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
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct RSLatch {
  output: Signal<bool>,
  set: Signal<bool>,
  reset: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl RSLatch {
  pub fn new(
    set: &Signal<bool>,
    reset: &Signal<bool>,
  ) -> Self {
    RSLatch {
      output: new_signal(false),
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
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct JKFlipFlop {
  output: Signal<bool>,
  j_input: Signal<bool>,
  k_input: Signal<bool>,
  clock: Signal<bool>,
  last_clock: bool,
}
////////////////////////////////////////////////////////////////////////////////
impl JKFlipFlop {
  pub fn new(
    j_input: &Signal<bool>,
    k_input: &Signal<bool>,
    clock: &Signal<bool>,
  ) -> Self {
    JKFlipFlop {
      output: new_signal(false),
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
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct DFlipFlop {
  output: Signal<bool>,
  input: Signal<bool>,
  clock: Signal<bool>,
  last_clock: bool,
}
////////////////////////////////////////////////////////////////////////////////
impl DFlipFlop {
  pub fn new(
    input: &Signal<bool>,
    clock: &Signal<bool>,
  ) -> Self {
    DFlipFlop {
      output: new_signal(false),
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
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct TFlipFlop {
  output: Signal<bool>,
  input: Signal<bool>,
  clock: Signal<bool>,
  last_clock: bool,
}
////////////////////////////////////////////////////////////////////////////////
impl TFlipFlop {
  pub fn new(
    input: &Signal<bool>,
    clock: &Signal<bool>,
  ) -> Self {
    TFlipFlop {
      output: new_signal(false),
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
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}
