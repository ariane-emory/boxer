use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LogicalOr {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicalOr {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicalOr {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for LogicalOr {
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
pub struct LogicalAnd {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicalAnd {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicalAnd {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for LogicalAnd {
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
pub struct LogicalNot {
  pub output: Signal<bool>,
  input: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicalNot {
  pub fn new(input: &Signal<bool>) -> Self {
    LogicalNot {
      output: new_signal(false),
      input: Rc::clone(input),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for LogicalNot {
  fn step(&mut self) {
    self.output.borrow_mut().set(!*self.input.borrow().read());
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
struct LogicalXor {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicalXor {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicalXor {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for LogicalXor {
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
struct LogicalNor {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicalNor {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicalNor {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for LogicalNor {
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
