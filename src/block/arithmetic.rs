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
