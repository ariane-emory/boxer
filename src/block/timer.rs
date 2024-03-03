use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-> 'TON' block, which delays a rise by a fixed number
// of cycles.
pub struct TON {
  output: OutputSignalRef<bool>,
  count_output: OutputSignalRef<usize>,
  delay: OutputSignalRef<usize>,
  reset: OutputSignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl TON {
  pub fn new(
    delay: &OutputSignalRef<usize>,
    reset: &OutputSignalRef<bool>,
  ) -> Self {
    TON {
      output: new_signal(false),
      count_output: new_signal(0),
      delay: Rc::clone(delay),
      reset: Rc::clone(reset),
    }
  }

  pub fn count_output(&self) -> &OutputSignalRef<usize> {
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
    } else if self.output.read() {
      self.count_output.set(self.count_output.read() + 1);
    } else {
      self.count_output.set(0);
    }

    if self.count_output.read() >= self.delay.read() {
      self.output.set(true);
    } else {
      self.output.set(false);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for TON {
  fn output(&self) -> &OutputSignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-> 'TOF' block, which delays a fall by a fixed number
// of cycles.
pub struct TOF {
  output: OutputSignalRef<bool>,
  count_output: OutputSignalRef<usize>,
  delay: OutputSignalRef<usize>,
  reset: OutputSignalRef<bool>,
}
////////////////////////////////////////////////////////////////////////////////
impl TOF {
  pub fn new(
    delay: &OutputSignalRef<usize>,
    reset: &OutputSignalRef<bool>,
  ) -> Self {
    TOF {
      output: new_signal(false),
      count_output: new_signal(0),
      delay: Rc::clone(delay),
      reset: Rc::clone(reset),
    }
  }

  pub fn count_output(&self) -> &OutputSignalRef<usize> {
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
    } else if !self.output.read() {
      self.count_output.set(self.count_output.read() + 1);
    } else {
      self.count_output.set(0);
    }

    if self.count_output.read() >= self.delay.read() {
      self.output.set(false);
    } else {
      self.output.set(true);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for TOF {
  fn output(&self) -> &OutputSignalRef<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-3 'TP' block, which holds it's input for a set number
// of steps after it rises.
struct TP {
  output: OutputSignalRef<bool>,
  count_output: OutputSignalRef<usize>,
  input: OutputSignalRef<bool>,
  count_from: OutputSignalRef<usize>,
}
////////////////////////////////////////////////////////////////////////////////
impl TP {
  pub fn new(
    input: &OutputSignalRef<bool>,
    count_from: &OutputSignalRef<usize>,
  ) -> Self {
    TP {
      output: new_signal(false),
      count_output: new_signal(0),
      input: Rc::clone(input),
      count_from: Rc::clone(count_from),
    }
  }

  pub fn count_output(&self) -> &OutputSignalRef<usize> {
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
    } else {
      self.output.set(false);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl HasOutputSignal<bool> for TP {
  fn output(&self) -> &OutputSignalRef<bool> {
    &self.output
  }
}
