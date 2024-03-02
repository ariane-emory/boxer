use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SquareWave {
  output: Signal<bool>,
  period: Signal<usize>,
  count: usize,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HasSignal<bool> for SquareWave {
  fn step(&mut self) {
    let last_output = *self.output.borrow().read();
    let period = *self.period.borrow().read();

    self.count = self.count + 1;

    if self.count >= period {
      self.output.borrow_mut().set(!last_output);
      self.count = 0;
    }
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}
