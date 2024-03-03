use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct SquareWave {
  output: SignalRef<bool>,
  period: SignalRef<usize>,
  count: usize,
}
////////////////////////////////////////////////////////////////////////////////
impl SquareWave {
  pub fn new(period: &SignalRef<usize>) -> Self {
    SquareWave {
      output: new_signal_ref(false),
      period: Rc::clone(period),
      count: 0,
    }
  }

  pub fn period(&self) -> &SignalRef<usize> {
    &self.period
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for SquareWave {
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
impl SteppableWithOutputSignal<bool> for SquareWave {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}
