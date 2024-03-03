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


////////////////////////////////////////////////////////////////////////////////
pub struct UsizeToF64 {
  output: SignalRef<f64>,
  input: SignalRef<usize>,
}
////////////////////////////////////////////////////////////////////////////////
impl UsizeToF64 {
  pub fn new(input: &SignalRef<usize>) -> Self {
    let mut r = UsizeToF64 {
      output: new_signal_ref(0.0),
      input: Rc::clone(input),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for UsizeToF64 {
  fn step(&mut self) {
    self.output.set(self.input.read() as f64);
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<f64> for UsizeToF64 {
  fn output(&self) -> &SignalRef<f64> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct F64ToUsize {
  output: SignalRef<usize>,
  input: SignalRef<f64>,
}
////////////////////////////////////////////////////////////////////////////////
impl F64ToUsize {
  pub fn new(input: &SignalRef<f64>) -> Self {
    let mut r = F64ToUsize {
      output: new_signal_ref(0),
      input: Rc::clone(input),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for F64ToUsize {
  fn step(&mut self) {
    let input = self.input.read();

    // panic if the value is too large to cast to usize:
    if input > usize::max_value() as f64 {
      panic!("F64ToUsize: input is too large to cast to usize");
    }

    self.output.set(self.input.read() as usize);
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<usize> for F64ToUsize {
  fn output(&self) -> &SignalRef<usize> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct F64ToIsize {
  output: SignalRef<isize>,
  input: SignalRef<f64>,
}
////////////////////////////////////////////////////////////////////////////////
impl F64ToIsize {
  pub fn new(input: &SignalRef<f64>) -> Self {
    let mut r = F64ToIsize {
      output: new_signal_ref(0),
      input: Rc::clone(input),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for F64ToIsize {
  fn step(&mut self) {
    let input = self.input.read();

    // panic if the value is too large to cast to isize:
    if input > isize::max_value() as f64 {
      panic!("F64ToIsize: input is too large to cast to isize");
    }

    self.output.set(self.input.read() as isize);
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<isize> for F64ToIsize {
  fn output(&self) -> &SignalRef<isize> {
    &self.output
  }
}
