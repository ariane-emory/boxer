use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathAdd<T: std::ops::Add<Output = T> + Copy + Default> {
  pub output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Add<Output = T> + Copy + Default> MathAdd<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    MathAdd {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Add<Output = T> + Copy + Default> Block<T> for MathAdd<T> {
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
pub struct MathSub<T: std::ops::Sub<Output = T> + Copy + Default> {
  pub output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Sub<Output = T> + Copy + Default> MathSub<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    MathSub {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Sub<Output = T> + Copy + Default> Block<T> for MathSub<T> {
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
pub struct MathMul<T: std::ops::Mul<Output = T> + Copy + Default> {
  pub output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> MathMul<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    MathMul {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> Block<T> for MathMul<T> {
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
pub struct MathDiv<T: std::ops::Div<Output = T> + Copy + Default> {
  pub output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> MathDiv<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    MathDiv {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> Block<T> for MathDiv<T> {
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
pub struct MathMod<T: std::ops::Rem<Output = T> + Copy + Default> {
  pub output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Rem<Output = T> + Copy + Default> MathMod<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    MathMod {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Rem<Output = T> + Copy + Default> Block<T> for MathMod<T> {
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
pub struct MathLShift {
  pub output: Signal<usize>,
  input_value: Signal<usize>,
  input_shift: Signal<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl MathLShift {
  pub fn new(input_value: &Signal<usize>, input_shift: &Signal<usize>) -> Self {
    MathLShift {
      output: Rc::new(RefCell::new(BlockOutput::new(0))),
      input_value: Rc::clone(input_value),
      input_shift: Rc::clone(input_shift),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<usize> for MathLShift {
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
pub struct MathRShift {
  pub output: Signal<usize>,
  input_value: Signal<usize>,
  input_shift: Signal<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl MathRShift {
  pub fn new(input_value: &Signal<usize>, input_shift: &Signal<usize>) -> Self {
    MathRShift {
      output: Rc::new(RefCell::new(BlockOutput::new(0))),
      input_value: Rc::clone(input_value),
      input_shift: Rc::clone(input_shift),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<usize> for MathRShift {
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
