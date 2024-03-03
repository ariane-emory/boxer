use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct Select<T: Copy> {
  output: SignalRef<T>,
  which: SignalRef<bool>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Select<T> {
  pub fn new(
    which: &SignalRef<bool>,
    left: &SignalRef<T>,
    right: &SignalRef<T>,
  ) -> Self {
    let mut r = Select {
      output: new_signal(left.read()),
      which: Rc::clone(which),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Steppable for Select<T> {
  fn step(&mut self) {
    if self.which.read() {
      self.output.set(self.right.read());
    } else {
      self.output.set(self.left.read());
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> HasSignal<T> for Select<T> {
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Max<T: Copy + PartialOrd> {
  output: SignalRef<T>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Max<T> {
  pub fn new(
    left: &SignalRef<T>,
    right: &SignalRef<T>,
  ) -> Self {
    Max {
      output: new_signal(left.read()),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Steppable for Max<T> {
  fn step(&mut self) {
    if self.right.read() > self.left.read() {
      self.output.set(self.right.read());
    } else {
      self.output.set(self.left.read());
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> HasSignal<T> for Max<T> {
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}

////////////////////////////////////////////////////////////////////////////////
pub struct Min<T: Copy + PartialOrd> {
  output: SignalRef<T>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Min<T> {
  pub fn new(
    left: &SignalRef<T>,
    right: &SignalRef<T>,
  ) -> Self {
    Min {
      output: new_signal(left.read()),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Steppable for Min<T> {
  fn step(&mut self) {
    if self.right.read() < self.left.read() {
      self.output.set(self.right.read());
    } else {
      self.output.set(self.left.read());
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> HasSignal<T> for Min<T> {
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}

////////////////////////////////////////////////////////////////////////////////
pub struct Limit<T: Copy + PartialOrd> {
  output: SignalRef<T>,
  input: SignalRef<T>,
  min: SignalRef<T>,
  max: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Limit<T> {
  pub fn new(
    input: &SignalRef<T>,
    min: &SignalRef<T>,
    max: &SignalRef<T>,
  ) -> Self {
    Limit {
      output: new_signal(input.read()),
      input: Rc::clone(input),
      min: Rc::clone(min),
      max: Rc::clone(max),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Steppable for Limit<T> {
  fn step(&mut self) {
    if self.input.read() < self.min.read() {
      self.output.set(self.min.read());
    } else if self.input.read() > self.max.read() {
      self.output.set(self.max.read());
    } else {
      self.output.set(self.input.read());
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> HasSignal<T> for Limit<T> {
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}
