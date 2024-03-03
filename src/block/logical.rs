use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct Or {
  output: OutputSignalRef<bool>,
  left: OutputSignalRef<bool>,
  right: OutputSignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Or {
  pub fn new(
    left: &OutputSignalRef<bool>,
    right: &OutputSignalRef<bool>,
  ) -> Self {
    let mut r = Or {
      output: new_signal(false),
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
impl HasOutputSignal<bool> for Or {
  fn output(&self) -> &OutputSignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct And {
  output: OutputSignalRef<bool>,
  left: OutputSignalRef<bool>,
  right: OutputSignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl And {
  pub fn new(
    left: &OutputSignalRef<bool>,
    right: &OutputSignalRef<bool>,
  ) -> Self {
    let mut r = And {
      output: new_signal(false),
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
impl HasOutputSignal<bool> for And {
  fn output(&self) -> &OutputSignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
struct Xor {
  output: OutputSignalRef<bool>,
  left: OutputSignalRef<bool>,
  right: OutputSignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Xor {
  pub fn new(
    left: &OutputSignalRef<bool>,
    right: &OutputSignalRef<bool>,
  ) -> Self {
    let mut r = Xor {
      output: new_signal(false),
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
impl HasOutputSignal<bool> for Xor {
  fn output(&self) -> &OutputSignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
struct Nor {
  output: OutputSignalRef<bool>,
  left: OutputSignalRef<bool>,
  right: OutputSignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Nor {
  pub fn new(
    left: &OutputSignalRef<bool>,
    right: &OutputSignalRef<bool>,
  ) -> Self {
    let mut r = Nor {
      output: new_signal(false),
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
impl HasOutputSignal<bool> for Nor {
  fn output(&self) -> &OutputSignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Not {
  output: OutputSignalRef<bool>,
  input: OutputSignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Not {
  pub fn new(input: &OutputSignalRef<bool>) -> Self {
    let mut r = Not {
      output: new_signal(false),
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
impl HasOutputSignal<bool> for Not {
  fn output(&self) -> &OutputSignalRef<bool> {
    &self.output
  }
}
