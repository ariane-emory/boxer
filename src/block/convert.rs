use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct IsizeToUsize {
  output: SignalRef<usize>,
  input: SignalRef<isize>,
}
////////////////////////////////////////////////////////////////////////////////
impl IsizeToUsize {
  pub fn new(input: &SignalRef<isize>) -> Self {
    let mut r = IsizeToUsize {
      output: new_signal_ref(0),
      input: Rc::clone(input),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for IsizeToUsize {
  fn step(&mut self) {
    let input = self.input.read();

    // panic if the number is negative:
    if input < 0 {
      panic!("IsizeToUsize: input is negative");
    }

    self.output.set(input as usize);
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<usize> for IsizeToUsize {
  fn output(&self) -> &SignalRef<usize> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct UsizeToIsize {
  output: SignalRef<isize>,
  input: SignalRef<usize>,
}
////////////////////////////////////////////////////////////////////////////////
impl UsizeToIsize {
  pub fn new(input: &SignalRef<usize>) -> Self {
    let mut r = UsizeToIsize {
      output: new_signal_ref(0),
      input: Rc::clone(input),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for UsizeToIsize {
  fn step(&mut self) {
    let input = self.input.read();

    // panic if the value is to large to cast to usize:
    if input > isize::max_value() as usize {
      panic!("UsizeToIsize: input is too large to cast to isize");
    }

    self.output.set(self.input.read() as isize);
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<isize> for UsizeToIsize {
  fn output(&self) -> &SignalRef<isize> {
    &self.output
  }
}
