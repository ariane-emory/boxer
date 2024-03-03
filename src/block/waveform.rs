use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct SquareWave {
  output: Signal<bool>,
  period: Signal<usize>,
  count: usize,
}
////////////////////////////////////////////////////////////////////////////////
impl SquareWave {
  pub fn new(period: &Signal<usize>) -> Self {
    SquareWave {
      output: new_signal(false),
      period: Rc::clone(period),
      count: 0,
    }
  }

  pub fn period(&self) -> &Signal<usize> {
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
impl HasOutputSignal<bool> for SquareWave {
  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}
