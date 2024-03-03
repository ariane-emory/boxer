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


////////////////////////////////////////////////////////////////////////////////
pub struct TriangleWave {
  output: SignalRef<isize>,
  period: SignalRef<usize>,
  count: usize,
  up: bool,
}
////////////////////////////////////////////////////////////////////////////////
impl TriangleWave {
  pub fn new(period: &SignalRef<usize>) -> Self {
    TriangleWave {
      output: new_signal_ref(0),
      period: Rc::clone(period),
      count: 0,
      up: true,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for TriangleWave {
  fn step(&mut self) {
    let period = self.period.read();

    if self.up {
      self.count = self.count + 1;
    } else {
      self.count = self.count - 1;
    }

    if self.count >= period {
      self.up = false;
    } else if self.count <= 0 {
      self.up = true;
    }

    self.output.set(self.count);
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<isize> for TriangleWave {
  fn output(&self) -> &SignalRef<isize> {
    &self.output
  }
}
