use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Add<T: std::ops::Add<Output = T> + Copy + Default> {
  output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Add<Output = T> + Copy + Default> Add<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    let mut r = Add {
      output: Rc::new(RefCell::new(SignalOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Add<Output = T> + Copy + Default> HasSignal<T> for Add<T> {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() + *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Sub<T: std::ops::Sub<Output = T> + Copy + Default> {
  output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Sub<Output = T> + Copy + Default> Sub<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    let mut r = Sub {
      output: Rc::new(RefCell::new(SignalOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Sub<Output = T> + Copy + Default> HasSignal<T> for Sub<T> {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() - *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Mul<T: std::ops::Mul<Output = T> + Copy + Default> {
  output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> Mul<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    let mut r = Mul {
      output: Rc::new(RefCell::new(SignalOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> HasSignal<T> for Mul<T> {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() * *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Div<T: std::ops::Div<Output = T> + Copy + Default> {
  output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> Div<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    let mut r = Div {
      output: Rc::new(RefCell::new(SignalOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> HasSignal<T> for Div<T> {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() / *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Mod<T: std::ops::Rem<Output = T> + Copy + Default> {
  output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Rem<Output = T> + Copy + Default> Mod<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    let mut r = Mod {
      output: Rc::new(RefCell::new(SignalOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Rem<Output = T> + Copy + Default> HasSignal<T> for Mod<T> {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() % *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LShift {
  output: Signal<usize>,
  input_value: Signal<usize>,
  input_shift: Signal<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LShift {
  pub fn new(input_value: &Signal<usize>, input_shift: &Signal<usize>) -> Self {
    let mut r = LShift {
      output: Rc::new(RefCell::new(SignalOutput::new(0))),
      input_value: Rc::clone(input_value),
      input_shift: Rc::clone(input_shift),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HasSignal<usize> for LShift {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.input_value.borrow().read() << *self.input_shift.borrow().read());
  }

  fn output(&self) -> &Signal<usize> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RShift {
  output: Signal<usize>,
  input_value: Signal<usize>,
  input_shift: Signal<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RShift {
  pub fn new(input_value: &Signal<usize>, input_shift: &Signal<usize>) -> Self {
    let mut r = RShift {
      output: Rc::new(RefCell::new(SignalOutput::new(0))),
      input_value: Rc::clone(input_value),
      input_shift: Rc::clone(input_shift),
    };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HasSignal<usize> for RShift {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.input_value.borrow().read() >> *self.input_shift.borrow().read());
  }

  fn output(&self) -> &Signal<usize> {
    &self.output
  }
}
