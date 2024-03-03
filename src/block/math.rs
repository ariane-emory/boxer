use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct Add<T: std::ops::Add<Output = T> + Copy + Default> {
  output: SignalRef<T>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Add<Output = T> + Copy + Default> Add<T> {
  pub fn new(left: &SignalRef<T>, right: &SignalRef<T>) -> Self {
    let mut r = Add {
      output: new_signal_ref(Default::default()),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Add<Output = T> + Copy + Default> Steppable for Add<T> {
  fn step(&mut self) {
    self.output.set(self.left.read() + self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Add<Output = T> + Copy + Default> SteppableWithOutputSignal<T>
  for Add<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Sub<T: std::ops::Sub<Output = T> + Copy + Default> {
  output: SignalRef<T>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Sub<Output = T> + Copy + Default> Sub<T> {
  pub fn new(left: &SignalRef<T>, right: &SignalRef<T>) -> Self {
    let mut r = Sub {
      output: new_signal_ref(Default::default()),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Sub<Output = T> + Copy + Default> Steppable for Sub<T> {
  fn step(&mut self) {
    self.output.set(self.left.read() - self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Sub<Output = T> + Copy + Default> SteppableWithOutputSignal<T>
  for Sub<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Mul<T: std::ops::Mul<Output = T> + Copy + Default> {
  output: SignalRef<T>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> Mul<T> {
  pub fn new(left: &SignalRef<T>, right: &SignalRef<T>) -> Self {
    let mut r = Mul {
      output: new_signal_ref(Default::default()),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> Steppable for Mul<T> {
  fn step(&mut self) {
    self.output.set(self.left.read() * self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> SteppableWithOutputSignal<T>
  for Mul<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Div<T: std::ops::Div<Output = T> + Copy + Default> {
  output: SignalRef<T>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> Div<T> {
  pub fn new(left: &SignalRef<T>, right: &SignalRef<T>) -> Self {
    let mut r = Div {
      output: new_signal_ref(Default::default()),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> Steppable for Div<T> {
  fn step(&mut self) {
    self.output.set(self.left.read() / self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> SteppableWithOutputSignal<T>
  for Div<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Mod<T: std::ops::Rem<Output = T> + Copy + Default> {
  output: SignalRef<T>,
  left: SignalRef<T>,
  right: SignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Rem<Output = T> + Copy + Default> Mod<T> {
  pub fn new(left: &SignalRef<T>, right: &SignalRef<T>) -> Self {
    let mut r = Mod {
      output: new_signal_ref(Default::default()),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Rem<Output = T> + Copy + Default> Steppable for Mod<T> {
  fn step(&mut self) {
    self.output.set(self.left.read() % self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Rem<Output = T> + Copy + Default> SteppableWithOutputSignal<T>
  for Mod<T>
{
  fn output(&self) -> &SignalRef<T> {
    &self.output
  }
}


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
pub struct Abs {
  output: SignalRef<usize>,
  input: SignalRef<isize>,
}
////////////////////////////////////////////////////////////////////////////////
impl Abs {
  pub fn new(input: &SignalRef<isize>) -> Self {
    let mut r = Abs {
      output: new_signal_ref(0),
      input: Rc::clone(input),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Steppable for Abs {
  fn step(&mut self) {
    self.output.set((self.input.read()).abs() as usize);
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<usize> for Abs {
  fn output(&self) -> &SignalRef<usize> {
    &self.output
  }
}


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

    // this should panic if the number is negative:
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
    self.output.set(self.input.read() as isize);
  }
}
////////////////////////////////////////////////////////////////////////////////
impl SteppableWithOutputSignal<isize> for UsizeToIsize {
  fn output(&self) -> &SignalRef<isize> {
    &self.output
  }
}
