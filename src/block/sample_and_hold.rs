use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
struct SampleAndHold<T: Copy + Default> {
  output: Signal<T>,
  set: Signal<bool>,
  reset: Signal<bool>,
  input: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> SampleAndHold<T> {
  pub fn new(set: &Signal<bool>, reset: &Signal<bool>, input: &Signal<T>) -> Self {
    SampleAndHold {
      output: new_signal(Default::default()),
      set: Rc::clone(set),
      reset: Rc::clone(reset),
      input: Rc::clone(input),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Block<T> for SampleAndHold<T> {
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
