use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct LShift {
  output: SignalRef<usize>,
  input_value: SignalRef<usize>,
  input_shift: SignalRef<usize>,
}
////////////////////////////////////////////////////////////////////////////////
impl LShift {
  pub fn new(
    input_value: &SignalRef<usize>,
    input_shift: &SignalRef<usize>,
  ) -> Self {
    let mut r = LShift {
      output: new_signal_ref(0),
      input_value: Rc::clone(input_value),
      input_shift: Rc::clone(input_shift),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for LShift {
  fn step(&mut self) {
    self
      .output
      .set(self.input_value.read() << self.input_shift.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<usize> for LShift {
  fn output(&self) -> &SignalRef<usize> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct RShift {
  output: SignalRef<usize>,
  input_value: SignalRef<usize>,
  input_shift: SignalRef<usize>,
}
////////////////////////////////////////////////////////////////////////////////
impl RShift {
  pub fn new(
    input_value: &SignalRef<usize>,
    input_shift: &SignalRef<usize>,
  ) -> Self {
    let mut r = RShift {
      output: new_signal_ref(0),
      input_value: Rc::clone(input_value),
      input_shift: Rc::clone(input_shift),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for RShift {
  fn step(&mut self) {
    self
      .output
      .set(self.input_value.read() >> self.input_shift.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<usize> for RShift {
  fn output(&self) -> &SignalRef<usize> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Neg<T: std::ops::Neg<Output = T> + Copy + Default> {
  output: SignalRef<T>,
  input: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Neg<Output = T> + Copy + Default> Neg<T> {
  pub fn new(input: &SignalRef<T>) -> Self {
    let mut r = Neg {
      output: new_signal_ref(Default::default()),
      input: Rc::clone(input),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Neg<Output = T> + Copy + Default> Steppable for Neg<T> {
  fn step(&mut self) {
    self.output.set(-self.input.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Neg<Output = T> + Copy + Default> SteppableWithOutputSignal<T>
  for Neg<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Abs<
  T: std::ops::Neg<Output = T> + std::cmp::PartialOrd + Copy + Default,
> {
  output: SignalRef<T>,
  input: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Neg<Output = T> + std::cmp::PartialOrd + Copy + Default>
  Abs<T>
{
  pub fn new(input: &SignalRef<T>) -> Self {
    let mut r = Abs {
      output: new_signal_ref(Default::default()),
      input: Rc::clone(input),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Neg<Output = T> + std::cmp::PartialOrd + Copy + Default>
  Steppable for Abs<T>
{
  fn step(&mut self) {
    if self.input.read() < Default::default() {
      self.output.set(-self.input.read());
    }
    else {
      self.output.set(self.input.read());
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Neg<Output = T> + std::cmp::PartialOrd + Copy + Default>
  SteppableWithOutputSignal<T> for Abs<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Scale<
  T: std::ops::Add<Output = T>
    + std::ops::Sub<Output = T>
    + std::ops::Div<Output = T>
    + std::ops::Mul<Output = T>
    + Copy
    + Default,
> {
  output: SignalRef<T>,
  input: SignalRef<T>,
  in_min: SignalRef<T>,
  in_max: SignalRef<T>,
  out_min: SignalRef<T>,
  out_max: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<
    T: std::ops::Add<Output = T>
      + std::ops::Sub<Output = T>
      + std::ops::Div<Output = T>
      + std::ops::Mul<Output = T>
      + Copy
      + Default,
  > Scale<T>
{
  pub fn new(
    input: &SignalRef<T>,
    in_min: &SignalRef<T>,
    in_max: &SignalRef<T>,
    out_min: &SignalRef<T>,
    out_max: &SignalRef<T>,
  ) -> Self {
    let mut r = Scale {
      output: new_signal_ref(Default::default()),
      input: Rc::clone(input),
      in_min: Rc::clone(in_min),
      in_max: Rc::clone(in_max),
      out_min: Rc::clone(out_min),
      out_max: Rc::clone(out_max),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<
    T: std::ops::Add<Output = T>
      + std::ops::Sub<Output = T>
      + std::ops::Div<Output = T>
      + std::ops::Mul<Output = T>
      + Copy
      + Default,
  > Steppable for Scale<T>
{
  fn step(&mut self) {
    let in_min = self.in_min.read();
    let in_max = self.in_max.read();
    let out_min = self.out_min.read();
    let out_max = self.out_max.read();
    let input = self.input.read();

    self.output.set(
      (input - in_min) * (out_max - out_min) / (in_max - in_min) + out_min,
    );
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<
    T: std::ops::Add<Output = T>
      + std::ops::Sub<Output = T>
      + std::ops::Div<Output = T>
      + std::ops::Mul<Output = T>
      + Copy
      + Default,
  > SteppableWithOutputSignal<T> for Scale<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Clamp<T: std::cmp::PartialOrd + Copy + Default> {
  output: SignalRef<T>,
  input: SignalRef<T>,
  min: SignalRef<T>,
  max: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy + Default> Clamp<T> {
  pub fn new(
    input: &SignalRef<T>,
    min: &SignalRef<T>,
    max: &SignalRef<T>,
  ) -> Self {
    let mut r = Clamp {
      output: new_signal_ref(Default::default()),
      input: Rc::clone(input),
      min: Rc::clone(min),
      max: Rc::clone(max),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy + Default> Steppable for Clamp<T> {
  fn step(&mut self) {
    let input = self.input.read();
    let min = self.min.read();
    let max = self.max.read();

    if input < min {
      self.output.set(min);
    }
    else if input > max {
      self.output.set(max);
    }
    else {
      self.output.set(input);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy + Default> SteppableWithOutputSignal<T>
  for Clamp<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
// The Wrap block wraps  a value within a range. If the value is less than the
// minimum, it will be set to the maximum. If the value is greater than the
// maximum, it will be set to the minimum. If the value is within the range, it
// will be left unchanged.
pub struct Wrap<
  T: std::ops::Add<Output = T>
    + std::ops::Sub<Output = T>
    + std::ops::Rem<Output = T>
    + std::cmp::PartialOrd
    + Copy
    + Default,
> {
  output: SignalRef<T>,
  input: SignalRef<T>,
  min: SignalRef<T>,
  max: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<
    T: std::ops::Add<Output = T>
      + std::ops::Sub<Output = T>
      + std::ops::Rem<Output = T>
      + std::cmp::PartialOrd
      + Copy
      + Default,
  > Wrap<T>
{
  pub fn new(
    input: &SignalRef<T>,
    min: &SignalRef<T>,
    max: &SignalRef<T>,
  ) -> Self {
    let mut r = Wrap {
      output: new_signal_ref(Default::default()),
      input: Rc::clone(input),
      min: Rc::clone(min),
      max: Rc::clone(max),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<
    T: std::ops::Add<Output = T>
      + std::ops::Sub<Output = T>
      + std::ops::Rem<Output = T>
      + std::cmp::PartialOrd
      + Copy
      + Default,
  > Steppable for Wrap<T>
{
  fn step(&mut self) {
    let input = self.input.read();
    let min = self.min.read();
    let max = self.max.read();

    if input < min {
      self.output.set(max - (min - input) % (max - min));
    }
    else if input > max {
      self.output.set(min + (input - min) % (max - min));
    }
    else {
      self.output.set(input);
    }
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<
    T: std::ops::Add<Output = T>
      + std::ops::Sub<Output = T>
      + std::ops::Rem<Output = T>
      + std::cmp::PartialOrd
      + Copy
      + Default,
  > SteppableWithOutputSignal<T> for Wrap<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Sqrt<T: std::ops::Mul<Output = T> + Copy + Default> {
  output: SignalRef<T>,
  input: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> Sqrt<T> {
  pub fn new(input: &SignalRef<T>) -> Self {
    let mut r = Sqrt {
      output: new_signal_ref(Default::default()),
      input: Rc::clone(input),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> Steppable for Sqrt<T> {
  fn step(&mut self) {
    self.output.set(self.input.read() * self.input.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> SteppableWithOutputSignal<T>
  for Sqrt<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Pow<T: std::ops::Mul<Output = T> + Copy + Default> {
  output: SignalRef<T>,
  input: SignalRef<T>,
  exponent: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> Pow<T> {
  pub fn new(input: &SignalRef<T>, exponent: &SignalRef<T>) -> Self {
    let mut r = Pow {
      output: new_signal_ref(Default::default()),
      input: Rc::clone(input),
      exponent: Rc::clone(exponent),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> Steppable for Pow<T> {
  fn step(&mut self) {
    self.output.set(self.input.read() * self.exponent.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> SteppableWithOutputSignal<T>
  for Pow<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Log<T: std::ops::Div<Output = T> + Copy + Default> {
  output: SignalRef<T>,
  input: SignalRef<T>,
  base: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> Log<T> {
  pub fn new(input: &SignalRef<T>, base: &SignalRef<T>) -> Self {
    let mut r = Log {
      output: new_signal_ref(Default::default()),
      input: Rc::clone(input),
      base: Rc::clone(base),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> Steppable for Log<T> {
  fn step(&mut self) {
    self.output.set(self.input.read() / self.base.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> SteppableWithOutputSignal<T>
  for Log<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}
