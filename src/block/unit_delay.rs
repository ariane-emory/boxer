use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
// This delays it's input by once cycle:
pub struct UnitDelay<T: Copy + Default> {
  output: SignalRef<T>,
  input: SignalRef<T>,
  previous: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> UnitDelay<T> {
  pub fn new(input: &SignalRef<T>) -> Self {
    UnitDelay {
      output: new_signal_ref(Default::default()),
      input: Rc::clone(input),
      previous: new_signal_ref(Default::default()),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Steppable for UnitDelay<T> {
  fn step(&mut self) {
    self.previous.set(self.input.read());
    self.output.set(self.previous.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> SteppableOutputSignal<T> for UnitDelay<T> {
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}
