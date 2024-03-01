#![allow(dead_code)]
pub mod cmp;
pub mod counter;
pub mod latch;
pub mod logic;
pub mod math;
pub mod random;
pub mod select;
pub mod timer;
pub mod trigger;
pub mod value;
pub mod jump;

pub use cmp::*;
pub use counter::*;
// pub use latch::*;
pub use logic::*;
pub use math::*;
pub use random::*;
pub use select::*;
pub use timer::*;
pub use trigger::*;
pub use value::*;
pub use jump::*;

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
