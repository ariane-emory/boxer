use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct Or {
  output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Or {
  pub fn new(
    left: &Signal<bool>,
    right: &Signal<bool>,
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
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() || *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for Or {
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct And {
  output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl And {
  pub fn new(
    left: &Signal<bool>,
    right: &Signal<bool>,
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
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() && *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for And {
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
struct Xor {
  output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Xor {
  pub fn new(
    left: &Signal<bool>,
    right: &Signal<bool>,
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
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() ^ *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for Xor {
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
struct Nor {
  output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Nor {
  pub fn new(
    left: &Signal<bool>,
    right: &Signal<bool>,
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
    self
      .output
      .borrow_mut()
      .set(!(*self.left.borrow().read() || *self.right.borrow().read()));
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for Nor {
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Not {
  output: Signal<bool>,
  input: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl Not {
  pub fn new(input: &Signal<bool>) -> Self {
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
    self.output.borrow_mut().set(!*self.input.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for Not {
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}
