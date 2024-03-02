use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct Value<T: Copy> {
  output: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Value<T> {
  pub fn new(value: T) -> Self {
    Value {
      output: Rc::new(RefCell::new(SignalOutput::new(value))),
    }
  }

  pub fn set(
    &mut self,
    value: T,
  ) {
    self.output.borrow_mut().set(value);
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Steppable for Value<T> {
  fn step(&mut self) {}
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> HasOutputSignal<T> for Value<T> {
  fn output(&self) -> &Signal<T> {
    &self.output
  }
}
