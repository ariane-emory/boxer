use crate::block::*;

////////////////////////////////////////////////////////////////////////////////
pub struct Add<T: std::ops::Add<Output = T> + Copy + Default> {
  output: OutputSignalRef<T>,
  left: OutputSignalRef<T>,
  right: OutputSignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Add<Output = T> + Copy + Default> Add<T> {
  pub fn new(
    left: &OutputSignalRef<T>,
    right: &OutputSignalRef<T>,
  ) -> Self {
    let mut r = Add {
      output: new_signal(Default::default()),
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
impl<T: std::ops::Add<Output = T> + Copy + Default> HasOutputSignal<T>
  for Add<T>
{
  fn output(&self) -> &OutputSignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Sub<T: std::ops::Sub<Output = T> + Copy + Default> {
  output: OutputSignalRef<T>,
  left: OutputSignalRef<T>,
  right: OutputSignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Sub<Output = T> + Copy + Default> Sub<T> {
  pub fn new(
    left: &OutputSignalRef<T>,
    right: &OutputSignalRef<T>,
  ) -> Self {
    let mut r = Sub {
      output: new_signal(Default::default()),
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
impl<T: std::ops::Sub<Output = T> + Copy + Default> HasOutputSignal<T>
  for Sub<T>
{
  fn output(&self) -> &OutputSignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Mul<T: std::ops::Mul<Output = T> + Copy + Default> {
  output: OutputSignalRef<T>,
  left: OutputSignalRef<T>,
  right: OutputSignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> Mul<T> {
  pub fn new(
    left: &OutputSignalRef<T>,
    right: &OutputSignalRef<T>,
  ) -> Self {
    let mut r = Mul {
      output: new_signal(Default::default()),
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
impl<T: std::ops::Mul<Output = T> + Copy + Default> HasOutputSignal<T>
  for Mul<T>
{
  fn output(&self) -> &OutputSignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Div<T: std::ops::Div<Output = T> + Copy + Default> {
  output: OutputSignalRef<T>,
  left: OutputSignalRef<T>,
  right: OutputSignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> Div<T> {
  pub fn new(
    left: &OutputSignalRef<T>,
    right: &OutputSignalRef<T>,
  ) -> Self {
    let mut r = Div {
      output: new_signal(Default::default()),
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
impl<T: std::ops::Div<Output = T> + Copy + Default> HasOutputSignal<T>
  for Div<T>
{
  fn output(&self) -> &OutputSignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Mod<T: std::ops::Rem<Output = T> + Copy + Default> {
  output: OutputSignalRef<T>,
  left: OutputSignalRef<T>,
  right: OutputSignalRef<T>,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Rem<Output = T> + Copy + Default> Mod<T> {
  pub fn new(
    left: &OutputSignalRef<T>,
    right: &OutputSignalRef<T>,
  ) -> Self {
    let mut r = Mod {
      output: new_signal(Default::default()),
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
impl<T: std::ops::Rem<Output = T> + Copy + Default> HasOutputSignal<T>
  for Mod<T>
{
  fn output(&self) -> &OutputSignalRef<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct LShift {
  output: OutputSignalRef<usize>,
  input_value: OutputSignalRef<usize>,
  input_shift: OutputSignalRef<usize>,
}
////////////////////////////////////////////////////////////////////////////////
impl LShift {
  pub fn new(
    input_value: &OutputSignalRef<usize>,
    input_shift: &OutputSignalRef<usize>,
  ) -> Self {
    let mut r = LShift {
      output: new_signal(0),
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
impl HasOutputSignal<usize> for LShift {
  fn output(&self) -> &OutputSignalRef<usize> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct RShift {
  output: OutputSignalRef<usize>,
  input_value: OutputSignalRef<usize>,
  input_shift: OutputSignalRef<usize>,
}
////////////////////////////////////////////////////////////////////////////////
impl RShift {
  pub fn new(
    input_value: &OutputSignalRef<usize>,
    input_shift: &OutputSignalRef<usize>,
  ) -> Self {
    let mut r = RShift {
      output: new_signal(0),
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
impl HasOutputSignal<usize> for RShift {
  fn output(&self) -> &OutputSignalRef<usize> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////
pub struct Abs {
  output: OutputSignalRef<usize>,
  input: OutputSignalRef<isize>,
}
////////////////////////////////////////////////////////////////////////////////
impl Abs {
  pub fn new(input: &OutputSignalRef<isize>) -> Self {
    let mut r = Abs {
      output: new_signal(0),
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
impl HasOutputSignal<usize> for Abs {
  fn output(&self) -> &OutputSignalRef<usize> {
    &self.output
  }
}
