use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct Or {
  output: SignalRef<bool>,
  left: SignalRef<bool>,
  right: SignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Or {
  pub fn new(
    left: &SignalRef<bool>,
    right: &SignalRef<bool>,
  ) -> Self {
    let mut r = Or {
      output: new_signal_ref(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for Or {
  fn step(&mut self) {
    self.output.set(self.left.read() || self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasSignal<bool> for Or {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct And {
  output: SignalRef<bool>,
  left: SignalRef<bool>,
  right: SignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl And {
  pub fn new(
    left: &SignalRef<bool>,
    right: &SignalRef<bool>,
  ) -> Self {
    let mut r = And {
      output: new_signal_ref(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for And {
  fn step(&mut self) {
    self.output.set(self.left.read() && self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasSignal<bool> for And {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
struct Xor {
  output: SignalRef<bool>,
  left: SignalRef<bool>,
  right: SignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Xor {
  pub fn new(
    left: &SignalRef<bool>,
    right: &SignalRef<bool>,
  ) -> Self {
    let mut r = Xor {
      output: new_signal_ref(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for Xor {
  fn step(&mut self) {
    self.output.set(self.left.read() ^ self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasSignal<bool> for Xor {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
struct Nor {
  output: SignalRef<bool>,
  left: SignalRef<bool>,
  right: SignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Nor {
  pub fn new(
    left: &SignalRef<bool>,
    right: &SignalRef<bool>,
  ) -> Self {
    let mut r = Nor {
      output: new_signal_ref(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for Nor {
  fn step(&mut self) {
    self.output.set(!(self.left.read() || self.right.read()));
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasSignal<bool> for Nor {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Not {
  output: SignalRef<bool>,
  input: SignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Not {
  pub fn new(input: &SignalRef<bool>) -> Self {
    let mut r = Not {
      output: new_signal_ref(false),
      input: Rc::clone(input),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for Not {
  fn step(&mut self) {
    self.output.set(!self.input.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasSignal<bool> for Not {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}
