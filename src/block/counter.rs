use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct UpCounter {
  output: SignalRef<usize>,
  at_max: SignalRef<bool>,
  clock: SignalRef<bool>,
  reset: SignalRef<bool>,
  max: SignalRef<usize>,
  last_clock_state: bool,
  last_reset_state: bool,
}
////////////////////////////////////////////////////////////////////////////////
impl UpCounter {
  pub fn new(
    clock: &SignalRef<bool>,
    reset: &SignalRef<bool>,
    max: &SignalRef<usize>,
  ) -> Self {
    UpCounter {
      output: new_signal_ref(0),
      at_max: new_signal_ref(false),
      clock: Rc::clone(clock),
      reset: Rc::clone(reset),
      max: Rc::clone(max),
      last_clock_state: false,
      last_reset_state: false,
    }
  }

  pub fn at_max(&self) -> &SignalRef<bool> {
    &self.at_max
  }

  pub fn at_max_value(&self) -> bool {
    self.at_max.read()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for UpCounter {
  fn step(&mut self) {
    let output_val = self.output.read();
    let max_val = self.max.read();
    let clock_val = self.clock.read();
    let reset_val = self.reset.read();
    let at_max = max_val == output_val;
    let last_clock_state_val = self.last_clock_state;
    let last_reset_state_val = self.last_reset_state;
    let clock_rose = clock_val && !last_clock_state_val;
    let reset_rose = reset_val && !last_reset_state_val;

    self.last_clock_state = clock_val;
    self.last_reset_state = reset_val;

    self.at_max.set(at_max);

    if reset_rose {
      // println!("Reset rose..");
      self.output.set(0);
      self.at_max.set(false);
    } else if at_max {
      // println!("At max!");
      return;
    } else if clock_rose {
      // println!("  Clock rose..");
      self.output.set(output_val + 1);
    } else {
      // println!("  Nothing interesting happened..");
    }
  }
}
///////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<usize> for UpCounter {
  fn output(&self) -> &SignalRef<usize> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub trait BorrowUpCounterRefAndGetAtMax {
  fn at_max(&self) -> SignalRef<bool>;

  fn at_max_value(&self) -> bool {
    self.at_max().read()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl BorrowUpCounterRefAndGetAtMax for RcRefCell<UpCounter> {
  fn at_max(&self) -> SignalRef<bool> {
    self.borrow().at_max().clone()
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct DownCounter {
  output: SignalRef<usize>,
  at_min: SignalRef<bool>,
  clock: SignalRef<bool>,
  reset: SignalRef<bool>,
  min: SignalRef<usize>,
  last_clock_state: bool,
  last_reset_state: bool,
}
////////////////////////////////////////////////////////////////////////////////
impl DownCounter {
  pub fn new(
    clock: &SignalRef<bool>,
    reset: &SignalRef<bool>,
    min: &SignalRef<usize>,
  ) -> Self {
    DownCounter {
      output: new_signal_ref(0),
      at_min: new_signal_ref(false),
      clock: Rc::clone(clock),
      reset: Rc::clone(reset),
      min: Rc::clone(min),
      last_clock_state: false,
      last_reset_state: false,
    }
  }

  pub fn at_min(&self) -> &SignalRef<bool> {
    &self.at_min
  }

  pub fn at_min_value(&self) -> bool {
    self.at_min.read()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for DownCounter {
  fn step(&mut self) {
    let output_val = self.output.read();
    let min_val = self.min.read();
    let clock_val = self.clock.read();
    let reset_val = self.reset.read();
    let at_min = output_val == min_val;
    let last_clock_state_val = self.last_clock_state;
    let last_reset_state_val = self.last_reset_state;
    let clock_rose = clock_val && !last_clock_state_val;
    let reset_rose = reset_val && !last_reset_state_val;

    self.last_clock_state = clock_val;
    self.last_reset_state = reset_val;

    self.at_min.set(at_min);

    if reset_rose {
      self.output.set(min_val);
      self.at_min.set(false);
    } else if at_min {
      return;
    } else if clock_rose {
      self.output.set(output_val - 1);
    }
  }
}
///////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<usize> for DownCounter {
  fn output(&self) -> &SignalRef<usize> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub trait BorrowDownCounterRefAndGetAtMin {
  fn at_min(&self) -> SignalRef<bool>;

  fn at_min_value(&self) -> bool {
    self.at_min().read()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl BorrowDownCounterRefAndGetAtMin for RcRefCell<DownCounter> {
  fn at_min(&self) -> SignalRef<bool> {
    self.borrow().at_min().clone()
  }
}
