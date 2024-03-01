use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SampleAndHold<T: Copy + Default> {
  output: Signal<T>,
  set: Signal<bool>,
  reset: Signal<bool>,
  input: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> SampleAndHold<T> {
  pub fn new(input: &Signal<T>, set: &Signal<bool>, reset: &Signal<bool>) -> Self {
    SampleAndHold {
      output: new_signal(*input.borrow().read()),
      input: Rc::clone(input),
      set: Rc::clone(set),
      reset: Rc::clone(reset),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Block<T> for SampleAndHold<T> {
  fn step(&mut self) {
    if *self.set.borrow().read() {
      self.output.borrow_mut().set(*self.input.borrow().read());
    } else if *self.reset.borrow().read() {
      self.output.borrow_mut().set(Default::default());
    }
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}
