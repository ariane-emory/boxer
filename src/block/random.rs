use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct RandomUsize {
  output: SignalRef<usize>,
}
////////////////////////////////////////////////////////////////////////////////
impl Default for RandomUsize {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomUsize {
  pub fn new() -> Self {
    let mut r = RandomUsize {
      output: new_signal_ref(0),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for RandomUsize {
  fn step(&mut self) {
    self.output.set(rand::random());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<usize> for RandomUsize {
  fn output(&self) -> &SignalRef<usize> {
    &self.output
  }
}
