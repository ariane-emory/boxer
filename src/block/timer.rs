use crate::block::*;


////////////////////////////////////////////////////////////////////////////////
pub trait BorrowTimerRefAndGetCountOutput {
  fn count_output(&self) -> SignalRef<usize>;

  fn count_output_value(&self) -> usize {
    self.count_output().read()
  }
}


////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-> 'TON' block, which delays a rise by a fixed number
// of cycles.
pub struct TON {
  output: SignalRef<bool>,
  count_output: SignalRef<usize>,
  delay: SignalRef<usize>,
  reset: SignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl TON {
  pub fn new(delay: &SignalRef<usize>, reset: &SignalRef<bool>) -> Self {
    TON {
      output: new_signal_ref(false),
      count_output: new_signal_ref(0),
      delay: Rc::clone(delay),
      reset: Rc::clone(reset),
    }
  }

  pub fn count_output(&self) -> &SignalRef<usize> {
    &self.count_output
  }

  pub fn count_output_value(&self) -> usize {
    self.count_output.read()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for TON {
  fn step(&mut self) {
    if self.reset.read() {
      self.count_output.set(0);
    }
    else if self.output.read() {
      self.count_output.set(self.count_output.read() + 1);
    }
    else {
      self.count_output.set(0);
    }

    if self.count_output.read() >= self.delay.read() {
      self.output.set(true);
    }
    else {
      self.output.set(false);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<bool> for TON {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}
////////////////////////////////////////////////////////////////////////////////
impl BorrowTimerRefAndGetCountOutput for TON {
  fn count_output(&self) -> SignalRef<usize> {
    Rc::clone(&self.count_output)
  }
}


////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-> 'TOF' block, which delays a fall by a fixed number
// of cycles.
pub struct TOF {
  output: SignalRef<bool>,
  count_output: SignalRef<usize>,
  delay: SignalRef<usize>,
  reset: SignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl TOF {
  pub fn new(delay: &SignalRef<usize>, reset: &SignalRef<bool>) -> Self {
    TOF {
      output: new_signal_ref(false),
      count_output: new_signal_ref(0),
      delay: Rc::clone(delay),
      reset: Rc::clone(reset),
    }
  }

  pub fn count_output(&self) -> &SignalRef<usize> {
    &self.count_output
  }

  pub fn count_output_value(&self) -> usize {
    self.count_output.read()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for TOF {
  fn step(&mut self) {
    if self.reset.read() {
      self.count_output.set(0);
    }
    else if !self.output.read() {
      self.count_output.set(self.count_output.read() + 1);
    }
    else {
      self.count_output.set(0);
    }

    if self.count_output.read() >= self.delay.read() {
      self.output.set(false);
    }
    else {
      self.output.set(true);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<bool> for TOF {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}
////////////////////////////////////////////////////////////////////////////////
impl BorrowTimerRefAndGetCountOutput for TOF {
  fn count_output(&self) -> SignalRef<usize> {
    Rc::clone(&self.count_output)
  }
}


////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-3 'TP' block, which holds it's input for a set number
// of steps after it rises.
struct TP {
  output: SignalRef<bool>,
  count_output: SignalRef<usize>,
  input: SignalRef<bool>,
  count_from: SignalRef<usize>,
}
////////////////////////////////////////////////////////////////////////////////
impl TP {
  pub fn new(input: &SignalRef<bool>, count_from: &SignalRef<usize>) -> Self {
    TP {
      output: new_signal_ref(false),
      count_output: new_signal_ref(0),
      input: Rc::clone(input),
      count_from: Rc::clone(count_from),
    }
  }

  pub fn count_output(&self) -> &SignalRef<usize> {
    &self.count_output
  }

  pub fn count_output_value(&self) -> usize {
    self.count_output.read()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for TP {
  fn step(&mut self) {
    if self.input.read() {
      self.count_output.set(self.count_from.read());
    }

    if self.count_output.read() > 0usize {
      self.output.set(true);
      self.count_output.set(self.count_output.read() - 1);
    }
    else {
      self.output.set(false);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<bool> for TP {
  fn output(&self) -> &SignalRef<bool> {
    &self.output
  }
}
////////////////////////////////////////////////////////////////////////////////
impl BorrowTimerRefAndGetCountOutput for TP {
  fn count_output(&self) -> SignalRef<usize> {
    Rc::clone(&self.count_output)
  }
}
