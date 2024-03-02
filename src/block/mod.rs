#![allow(dead_code)]
pub mod cmp;
pub mod counter;
pub mod feedback;
pub mod latch;
pub mod logical;
pub mod math;
pub mod random;
pub mod sample_and_hold;
pub mod select;
pub mod timer;
pub mod trigger;
pub mod unit_delay;
pub mod value;
pub mod waveform;

pub use cmp::*;
pub use counter::*;
pub use feedback::*;
pub use latch::*;
pub use logical::*;
pub use math::*;
pub use random::*;
pub use sample_and_hold::*;
pub use select::*;
pub use timer::*;
pub use trigger::*;
pub use unit_delay::*;
pub use value::*;
pub use waveform::*;

use std::cell::RefCell;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////
pub struct SignalOutput<T: Copy> {
  value: T,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> SignalOutput<T> {
  pub fn new(value: T) -> Self {
    SignalOutput { value }
  }

  pub fn read(&self) -> &T {
    &self.value
  }

  pub fn set(
    &mut self,
    value: T,
  ) {
    self.value = value;
  }
}


////////////////////////////////////////////////////////////////////////////////
pub type Signal<T> = Rc<RefCell<SignalOutput<T>>>;


////////////////////////////////////////////////////////////////////////////////
pub fn new_signal<T: Copy>(value: T) -> Signal<T> {
  Rc::new(RefCell::new(SignalOutput::new(value)))
}


////////////////////////////////////////////////////////////////////////////////
pub trait HasSignal<T: Copy>: Steppable {
  fn output(&self) -> &Signal<T>;

  fn output_value(&self) -> T {
    *self.output().borrow().read()
  }
}

////////////////////////////////////////////////////////////////////////////////
pub trait Steppable {
  fn step(&mut self);
}


////////////////////////////////////////////////////////////////////////////////
pub type RcRcSteppable = Rc<RefCell<dyn Steppable>>;

////////////////////////////////////////////////////////////////////////////////
pub fn push_onto_vec_rcrc_steppable<T: 'static + Steppable>(
  blocks: &mut Vec<RcRcSteppable>,
  item: &Rc<RefCell<T>>,
) {
  let steppable_item: RcRcSteppable =
    item.clone() as Rc<RefCell<dyn Steppable>>;

  blocks.push(steppable_item);
}
