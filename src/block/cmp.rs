use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct GreaterThan<T: std::cmp::PartialOrd + Copy> {
  output: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> GreaterThan<T> {
  pub fn new(left: &Signal<T>,
             right: &Signal<T>)
             -> Self {
    let mut r = GreaterThan { output: new_signal(false),
                              left: Rc::clone(left),
                              right: Rc::clone(right) };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> Steppable for GreaterThan<T> {
  fn step(&mut self) {
    self.output
        .borrow_mut()
        .set(*self.left.borrow().read() > *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> HasSignal<bool> for GreaterThan<T> {
  fn output(&self) -> &Signal<bool> { &self.output }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LessThan<T: std::cmp::PartialOrd + Copy> {
  output: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> LessThan<T> {
  pub fn new(left: &Signal<T>,
             right: &Signal<T>)
             -> Self {
    let mut r = LessThan { output: new_signal(false),
                           left: Rc::clone(left),
                           right: Rc::clone(right) };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> Steppable for LessThan<T> {
  fn step(&mut self) {
    self.output
        .borrow_mut()
        .set(*self.left.borrow().read() < *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> HasSignal<bool> for LessThan<T> {
  fn output(&self) -> &Signal<bool> { &self.output }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Equal<T: std::cmp::PartialEq + Copy> {
  output: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> Equal<T> {
  pub fn new(left: &Signal<T>,
             right: &Signal<T>)
             -> Self {
    let mut r = Equal { output: new_signal(false),
                        left: Rc::clone(left),
                        right: Rc::clone(right) };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> Steppable for Equal<T> {
  fn step(&mut self) {
    self.output
        .borrow_mut()
        .set(*self.left.borrow().read() == *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> HasSignal<bool> for Equal<T> {
  fn output(&self) -> &Signal<bool> { &self.output }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct NotEqual<T: std::cmp::PartialEq + Copy> {
  output: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> NotEqual<T> {
  pub fn new(left: &Signal<T>,
             right: &Signal<T>)
             -> Self {
    let mut r = NotEqual { output: new_signal(false),
                           left: Rc::clone(left),
                           right: Rc::clone(right) };

    r.step();
    r
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> Steppable for NotEqual<T> {
  fn step(&mut self) {
    self.output
        .borrow_mut()
        .set(*self.left.borrow().read() != *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialEq + Copy> HasSignal<bool> for NotEqual<T> {
  fn output(&self) -> &Signal<bool> { &self.output }
}
