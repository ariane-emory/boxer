use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-> 'TON' block, which delays a rise by a fixed number of cycles.
pub struct TON {
  output: Signal<bool>,
  count_output: Signal<usize>,
  delay: Signal<usize>,
  reset: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl TON {
  pub fn new(delay: &Signal<usize>,
             reset: &Signal<bool>)
             -> Self {
    TON { output: new_signal(false),
          count_output: new_signal(0),
          delay: Rc::clone(delay),
          reset: Rc::clone(reset) }
  }

  pub fn count_output(&self) -> &Signal<usize> { &self.count_output }

  pub fn count_output_value(&self) -> usize { *self.count_output.borrow().read() }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Steppable for TON {
  fn step(&mut self) {
    if *self.reset.borrow().read() {
      self.count_output.borrow_mut().set(0);
    } else if *self.output.borrow().read() {
      self.count_output
          .borrow_mut()
          .set(self.count_output.borrow().read() + 1);
    } else {
      self.count_output.borrow_mut().set(0);
    }

    if *self.count_output.borrow().read() >= *self.delay.borrow().read() {
      self.output.borrow_mut().set(true);
    } else {
      self.output.borrow_mut().set(false);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HasSignal<bool> for TON {
  fn output(&self) -> &Signal<bool> { &self.output }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-> 'TOF' block, which delays a fall by a fixed number of cycles.
pub struct TOF {
  output: Signal<bool>,
  count_output: Signal<usize>,
  delay: Signal<usize>,
  reset: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl TOF {
  pub fn new(delay: &Signal<usize>,
             reset: &Signal<bool>)
             -> Self {
    TOF { output: new_signal(false),
          count_output: new_signal(0),
          delay: Rc::clone(delay),
          reset: Rc::clone(reset) }
  }

  pub fn count_output(&self) -> &Signal<usize> { &self.count_output }

  pub fn count_output_value(&self) -> usize { *self.count_output.borrow().read() }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Steppable for TOF {
  fn step(&mut self) {
    if *self.reset.borrow().read() {
      self.count_output.borrow_mut().set(0);
    } else if !*self.output.borrow().read() {
      self.count_output
          .borrow_mut()
          .set(self.count_output.borrow().read() + 1);
    } else {
      self.count_output.borrow_mut().set(0);
    }

    if *self.count_output.borrow().read() >= *self.delay.borrow().read() {
      self.output.borrow_mut().set(false);
    } else {
      self.output.borrow_mut().set(true);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HasSignal<bool> for TOF {
  fn output(&self) -> &Signal<bool> { &self.output }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-3 'TP' block, which holds it's input for a set number of steps after it rises.
struct TP {
  output: Signal<bool>,
  count_output: Signal<usize>,
  input: Signal<bool>,
  count_from: Signal<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl TP {
  pub fn new(input: &Signal<bool>,
             count_from: &Signal<usize>)
             -> Self {
    TP { output: new_signal(false),
         count_output: new_signal(0),
         input: Rc::clone(input),
         count_from: Rc::clone(count_from) }
  }

  pub fn count_output(&self) -> &Signal<usize> { &self.count_output }

  pub fn count_output_value(&self) -> usize { *self.count_output.borrow().read() }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Steppable for TP {
  fn step(&mut self) {
    if *self.input.borrow().read() {
      self.count_output
          .borrow_mut()
          .set(*self.count_from.borrow().read());
    }

    if *self.count_output.borrow().read() > 0usize {
      self.output.borrow_mut().set(true);
      self.count_output
          .borrow_mut()
          .set(self.count_output.borrow().read() - 1);
    } else {
      self.output.borrow_mut().set(false);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HasSignal<bool> for TP {
  fn output(&self) -> &Signal<bool> { &self.output }
}
