use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
struct SRLatch {
  pub output: Signal<bool>,
  set: Signal<bool>,
  reset: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SRLatch {
  pub fn new(set: &Signal<bool>, reset: &Signal<bool>) -> Self {
    SRLatch {
      output: new_signal(false),
      set: Rc::clone(set),
      reset: Rc::clone(reset),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for SRLatch {
  fn step(&mut self) {
    if *self.set.borrow().read() {
      self.output.borrow_mut().set(true);
    } else if *self.reset.borrow().read() {
      self.output.borrow_mut().set(false);
    }
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
struct RSLatch {
  pub output: Signal<bool>,
  set: Signal<bool>,
  reset: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RSLatch {
  pub fn new(set: &Signal<bool>, reset: &Signal<bool>) -> Self {
    RSLatch {
      output: new_signal(false),
      set: Rc::clone(set),
      reset: Rc::clone(reset),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for RSLatch {
  fn step(&mut self) {
    if *self.reset.borrow().read() {
      self.output.borrow_mut().set(false);
    } else if *self.set.borrow().read() {
      self.output.borrow_mut().set(true);
    }
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}
