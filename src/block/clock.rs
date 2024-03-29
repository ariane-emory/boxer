use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct Clock {
  output: SignalRef<bool>,
  period: SignalRef<usize>,
  count: usize,
}
////////////////////////////////////////////////////////////////////////////////
impl Clock {
  pub fn new(period: &SignalRef<usize>) -> Self {
    Clock {
      output: new_signal_ref(false),
      period: Rc::clone(period),
      count: 0,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for Clock {
  fn step(&mut self) {
    let last_output = self.output.read();
    let period = self.period.read();

    self.count = self.count + 1;

    if self.count >= period {
      self.output.set(!last_output);
      self.count = 0;
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<bool> for Clock {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}
