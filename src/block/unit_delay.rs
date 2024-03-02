use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
// This delays it's input by once cycle:
pub struct UnitDelay<T: Copy + Default> {
  output: Signal<T>,
  input: Signal<T>,
  previous: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> UnitDelay<T> {
  pub fn new(input: &Signal<T>) -> Self {
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
    self.previous.borrow_mut().set(*self.input.borrow().read());
    self.output.borrow_mut().set(*self.previous.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> HasSignal<T> for UnitDelay<T> {
  fn output(&self) -> &Signal<T> {
    &self.output
  }
}
