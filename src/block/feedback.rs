use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct Feedback<T: Copy + Default> {
  output: OutputSignalRef<T>,
  input: Option<OutputSignalRef<T>>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Feedback<T> {
  pub fn new() -> Self {
    Feedback {
      output: new_signal(Default::default()),
      input: None,
    }
    //}
  }

  pub fn set_input(
    &mut self,
    input: &OutputSignalRef<T>,
  ) {
    self.input = Some(input.clone());
    self.step();
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Steppable for Feedback<T> {
  fn step(&mut self) {
    if let Some(input) = &self.input {
      self.output.set(input.read());
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> HasOutputSignal<T> for Feedback<T> {
  fn output(&self) -> &OutputSignalRef<T> {
    &self.output
  }
}
