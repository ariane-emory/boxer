use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RandomUsize {
  output: Signal<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RandomUsize {
  pub fn new() -> Self {
    let mut r = RandomUsize {
      output: new_signal(0),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Steppable for RandomUsize {
  fn step(&mut self) {
    self.output.borrow_mut().set(rand::random());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HasSignal<usize> for RandomUsize {
  fn output(&self) -> &Signal<usize> {
    &self.output
  }
}
