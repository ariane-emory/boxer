use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct SampleAndHold<T: Copy + Default> {
  output: OutputSignalRef<T>,
  set: OutputSignalRef<bool>,
  reset: OutputSignalRef<bool>,
  input: OutputSignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> SampleAndHold<T> {
  pub fn new(
    input: &OutputSignalRef<T>,
    set: &OutputSignalRef<bool>,
    reset: &OutputSignalRef<bool>,
  ) -> Self {
    SampleAndHold {
      output: new_signal(Default::default()),
      input: Rc::clone(input),
      set: Rc::clone(set),
      reset: Rc::clone(reset),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Steppable for SampleAndHold<T> {
  fn step(&mut self) {
    if self.set.read() {
      self.output.set(self.input.read());
    } else if self.reset.read() {
      self.output.set(Default::default());
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> HasOutputSignal<T> for SampleAndHold<T> {
  fn output(&self) -> &OutputSignalRef<T> {
    &self.output
  }
}
