use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct Feedback<T: Copy + Default> {
  output: SignalRef<T>,
  input: Option<SignalRef<T>>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + Default> Feedback<T> {
  pub fn new() -> Self {
    Feedback {
      output: new_signal_ref(Default::default()),
      input: None,
    }
    //}
  }

  pub fn set_input(&mut self, input: &SignalRef<T>) {
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
impl<T: Copy + Default> SteppableOutputSignal<T> for Feedback<T> {
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub trait BorrowFeedbackRefAndSetInput<U: Copy + Default> {
  fn set_input(&self, input: &SignalRef<U>);
}
////////////////////////////////////////////////////////////////////////////////
impl<U: Copy + Default> BorrowFeedbackRefAndSetInput<U>
  for RcRefCell<Feedback<U>>
where
  U: Copy + Default,
{
  fn set_input(&self, input: &SignalRef<U>) {
    self.borrow_mut().set_input(input);
  }
}
