#![allow(dead_code)]
pub mod logic;
pub mod math;
pub mod trigger;

pub use logic::*;
pub use math::*;
pub use trigger::*;

use std::cell::RefCell;
use std::rc::Rc;


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct BlockOutput<T: Copy> {
  value: T,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> BlockOutput<T> {
  pub fn new(value: T) -> Self {
    BlockOutput { value }
  }

  pub fn read(&self) -> &T {
    &self.value
  }

  pub fn set(&mut self, value: T) {
    self.value = value;
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub type Signal<T> = Rc<RefCell<BlockOutput<T>>>;


////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn new_signal<T: Copy>(value: T) -> Signal<T> {
  Rc::new(RefCell::new(BlockOutput::new(value)))
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Block<T: Copy> {
  fn step(&mut self);
  fn output(&self) -> &Signal<T>;
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Value<T: Copy> {
  pub output: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Value<T> {
  pub fn new(value: T) -> Self {
    Value {
      output: Rc::new(RefCell::new(BlockOutput::new(value))),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Block<T> for Value<T> {
  fn step(&mut self) {}

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Select<T: Copy> {
  pub output: Signal<T>,
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
      self.output.borrow_mut().set(*self.left.borrow().read());
    } else {
      self.output.borrow_mut().set(*self.right.borrow().read());
    }
  }

  fn output(&self) -> &Signal<T> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct GreaterThan<T: std::cmp::PartialOrd + Copy> {
  pub output: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> GreaterThan<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    GreaterThan {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> Block<bool> for GreaterThan<T> {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() > *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LessThan<T: std::cmp::PartialOrd + Copy> {
  pub output: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> LessThan<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    LessThan {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> Block<bool> for LessThan<T> {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() < *self.right.borrow().read());
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Counter {
  pub output: Signal<usize>,
  pub at_max: Signal<bool>,
  input: Signal<bool>,
  max: Signal<usize>,
  last_state: bool,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Counter {
  pub fn new(input: &Signal<bool>, max: &Signal<usize>) -> Self {
    Counter {
      output: new_signal(0),
      at_max: new_signal(false),
      input: Rc::clone(input),
      max: Rc::clone(max),
      last_state: false,
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RandomUsize {
  pub output: Signal<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RandomUsize {
  pub fn new() -> Self {
    RandomUsize {
      output: new_signal(0),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<usize> for RandomUsize {
  fn step(&mut self) {
    self.output.borrow_mut().set(rand::random());
  }

  fn output(&self) -> &Signal<usize> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
struct SRLatch {
  pub output: Signal<bool>,
  set: Signal<bool>,
  reset: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SRLatch {
  pub fn new(set: &Signal<bool>, reset: &Signal<bool>) -> Self {
    SRLatch {
      output: new_signal(false),
      set: Rc::clone(set),
      reset: Rc::clone(reset),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for SRLatch {
  fn step(&mut self) {
    if *self.set.borrow().read() {
      self.output.borrow_mut().set(true);
    } else if *self.reset.borrow().read() {
      self.output.borrow_mut().set(false);
    }
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
struct RSLatch {
  pub output: Signal<bool>,
  set: Signal<bool>,
  reset: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RSLatch {
  pub fn new(set: &Signal<bool>, reset: &Signal<bool>) -> Self {
    RSLatch {
      output: new_signal(false),
      set: Rc::clone(set),
      reset: Rc::clone(reset),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block<bool> for RSLatch {
  fn step(&mut self) {
    if *self.reset.borrow().read() {
      self.output.borrow_mut().set(false);
    } else if *self.set.borrow().read() {
      self.output.borrow_mut().set(true);
    }
  }

  fn output(&self) -> &Signal<bool> {
    &self.output
  }
}
