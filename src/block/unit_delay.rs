use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
// This delays it's input by once cycle:
pub struct UnitDelay<T: Copy + Default> {
  output: OutputSignalRef<T>,
  input: OutputSignalRef<T>,
  previous: OutputSignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> UnitDelay<T> {
  pub fn new(input: &OutputSignalRef<T>) -> Self {
    UnitDelay {
      output: new_signal(Default::default()),
      input: Rc::clone(input),
      previous: new_signal(Default::default()),
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
impl<T: Copy + Default> HasOutputSignal<T> for UnitDelay<T> {
  fn output(&self) -> &OutputSignalRef<T> {
    &self.output
  }
}
