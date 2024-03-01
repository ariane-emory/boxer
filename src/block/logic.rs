////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LogicOr {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicOr {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicOr {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for LogicOr {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() || *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LogicAnd {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicAnd {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicAnd {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for LogicAnd {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() && *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LogicNot {
  pub output: Signal<bool>,
  input: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicNot {
  pub fn new(input: &Signal<bool>) -> Self {
    LogicNot {
      output: new_signal(false),
      input: Rc::clone(input),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for LogicNot {
  fn step(&mut self) {
    self.output.borrow_mut().set(!*self.input.borrow().read());
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
struct LogicXor {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicXor {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicXor {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for LogicXor {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() ^ *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
struct LogicNor {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicNor {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicNor {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for LogicNor {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(!(*self.left.borrow().read() || *self.right.borrow().read()));
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}
