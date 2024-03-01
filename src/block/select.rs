use crate::block::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Select<T: Copy> {
  output: Signal<T>,
  which: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Select<T> {
  pub fn new(which: &Signal<bool>, left: &Signal<T>, right: &Signal<T>) -> Self {
    Select {
      output: new_signal(*left.borrow().read()),
      which: Rc::clone(which),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Block<T> for Select<T> {
  fn step(&mut self) {
    if *self.which.borrow().read() {
      self.output.borrow_mut().set(*self.right.borrow().read());
    } else {
      self.output.borrow_mut().set(*self.left.borrow().read());
    }
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Max<T: Copy + PartialOrd> {
  output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Max<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    Max {
      output: new_signal(*left.borrow().read()),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Block<T> for Max<T> {
  fn step(&mut self) {
    if *self.right.borrow().read() > *self.left.borrow().read() {
      self.output.borrow_mut().set(*self.right.borrow().read());
    } else {
      self.output.borrow_mut().set(*self.left.borrow().read());
    }
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Min<T: Copy + PartialOrd> {
  output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Min<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    Min {
      output: new_signal(*left.borrow().read()),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Block<T> for Min<T> {
  fn step(&mut self) {
    if *self.right.borrow().read() < *self.left.borrow().read() {
      self.output.borrow_mut().set(*self.right.borrow().read());
    } else {
      self.output.borrow_mut().set(*self.left.borrow().read());
    }
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Limit<T: Copy + PartialOrd> {
  output: Signal<T>,
  input: Signal<T>,
  min: Signal<T>,
  max: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Limit<T> {
  pub fn new(input: &Signal<T>, min: &Signal<T>, max: &Signal<T>) -> Self {
    Limit {
      output: new_signal(*input.borrow().read()),
      input: Rc::clone(input),
      min: Rc::clone(min),
      max: Rc::clone(max),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy + PartialOrd> Block<T> for Limit<T> {
  fn step(&mut self) {
    if *self.input.borrow().read() < *self.min.borrow().read() {
      self.output.borrow_mut().set(*self.min.borrow().read());
    } else if *self.input.borrow().read() > *self.max.borrow().read() {
      self.output.borrow_mut().set(*self.max.borrow().read());
    } else {
      self.output.borrow_mut().set(*self.input.borrow().read());
    }
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}
