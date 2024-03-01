use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SRLatch {
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
      println!("Set!");
      self.output.borrow_mut().set(true);
    } else if *self.reset.borrow().read() {
      println!("Reset!");
      self.output.borrow_mut().set(false);
    }
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RSLatch {
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


////////////////////////////////////////////////////////////////////////////////////////////////////
struct GenericRSLatch<T: Copy + Default> {
  pub output: Signal<T>,
  set: Signal<bool>,
  reset: Signal<bool>,
  input: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> GenericRSLatch<T> {
  pub fn new(set: &Signal<bool>, reset: &Signal<bool>, input: &Signal<T>) -> Self {
    GenericRSLatch {
      output: new_signal(Default::default()),
      set: Rc::clone(set),
      reset: Rc::clone(reset),
      input: Rc::clone(input),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Block<T> for GenericRSLatch<T> {
  fn step(&mut self) {
    if *self.reset.borrow().read() {
      self.output.borrow_mut().set(Default::default());
    } else if *self.set.borrow().read() {
      self.output.borrow_mut().set(*self.input.borrow().read());
    }
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}
