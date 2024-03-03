use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct GreaterThan<T: std::cmp::PartialOrd + Copy> {
  output: SignalRef<bool>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> GreaterThan<T> {
  pub fn new(
    left: &SignalRef<T>,
    right: &SignalRef<T>,
  ) -> Self {
    let mut r = GreaterThan {
      output: new_signal_ref(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> Steppable for GreaterThan<T> {
  fn step(&mut self) {
    self.output.set(self.left.read() > self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> SteppableWithOutputSignal<bool> for GreaterThan<T> {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct LessThan<T: std::cmp::PartialOrd + Copy> {
  output: SignalRef<bool>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> LessThan<T> {
  pub fn new(
    left: &SignalRef<T>,
    right: &SignalRef<T>,
  ) -> Self {
    let mut r = LessThan {
      output: new_signal_ref(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> Steppable for LessThan<T> {
  fn step(&mut self) {
    self.output.set(self.left.read() < self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> SteppableWithOutputSignal<bool> for LessThan<T> {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Equal<T: std::cmp::PartialEq + Copy> {
  output: SignalRef<bool>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> Equal<T> {
  pub fn new(
    left: &SignalRef<T>,
    right: &SignalRef<T>,
  ) -> Self {
    let mut r = Equal {
      output: new_signal_ref(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> Steppable for Equal<T> {
  fn step(&mut self) {
    self.output.set(self.left.read() == self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> SteppableWithOutputSignal<bool> for Equal<T> {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct NotEqual<T: std::cmp::PartialEq + Copy> {
  output: SignalRef<bool>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> NotEqual<T> {
  pub fn new(
    left: &SignalRef<T>,
    right: &SignalRef<T>,
  ) -> Self {
    let mut r = NotEqual {
      output: new_signal_ref(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> Steppable for NotEqual<T> {
  fn step(&mut self) {
    self.output.set(self.left.read() != self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> SteppableWithOutputSignal<bool> for NotEqual<T> {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}
