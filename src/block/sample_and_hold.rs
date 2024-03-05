use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct SampleAndHold<T: Copy + Default> {
  output: SignalRef<T>,
  set: SignalRef<bool>,
  reset: SignalRef<bool>,
  input: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> SampleAndHold<T> {
  pub fn new(
    input: &SignalRef<T>,
    set: &SignalRef<bool>,
    reset: &SignalRef<bool>,
  ) -> Self {
    SampleAndHold {
      output: new_signal_ref(Default::default()),
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
    }
    else if self.reset.read() {
      self.output.set(Default::default());
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> SteppableWithOutputSignal<T> for SampleAndHold<T> {
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}
