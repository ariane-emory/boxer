use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SquareWave {
  pub output: Signal<bool>,
  period: Signal<usize>,
  count: usize;
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SquareWave {
  pub fn new(period: &Signal<usize>) -> Self {
    SquareWave {
      period: Rc::clone(period),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for SquareWavey {
  fn step(&mut self) {
    let last_output = self.output.borrow().read();
    let period = self.period.borrow().read();

    if self.count == period {
      self.output.borrow_mut().set(! last_output);
      self.count = 0;
    } else {
      self.count = self.count + 1;
    }
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


