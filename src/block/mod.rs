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
pub trait Steppable {
  fn step(&mut self);
}


////////////////////////////////////////////////////////////////////////////////
pub type RcRcSteppable = Rc<RefCell<dyn Steppable>>;


////////////////////////////////////////////////////////////////////////////////
pub fn push_onto_vec_of_rcrc_steppable<T: 'static + Steppable>(
  blocks: &mut Vec<RcRcSteppable>,
  item: &Rc<RefCell<T>>,
) {
  let steppable_item: RcRcSteppable =
    item.clone() as Rc<RefCell<dyn Steppable>>;

  blocks.push(steppable_item);
}


////////////////////////////////////////////////////////////////////////////////
pub struct OutputSignal<T: Copy> {
  value: T,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> OutputSignal<T> {
  pub fn new(value: T) -> Self {
    OutputSignal { value }
  }

  pub fn read(&self) -> T {
    self.value
  }

  pub fn set(
    &mut self,
    value: T,
  ) {
    self.value = value;
  }
}


////////////////////////////////////////////////////////////////////////////////
pub type Signal<T> = Rc<RefCell<OutputSignal<T>>>;

pub trait BorrowAndRead<T: Copy> {
  fn read(&self) -> T;
}

impl<T> BorrowAndRead<T> for Signal<T>
where
  T: Copy,
{
  fn read(&self) -> T {
    self.borrow().read()
  }
}

////////////////////////////////////////////////////////////////////////////////
pub fn new_rcrc<T>(item: T) -> Rc<RefCell<T>> {
  Rc::new(RefCell::new(item))
}


////////////////////////////////////////////////////////////////////////////////
pub fn new_signal<T: Copy>(value: T) -> Signal<T> {
  new_rcrc(OutputSignal::new(value))
}

////////////////////////////////////////////////////////////////////////////////
pub trait HasOutputSignal<T: Copy>: Steppable {
  fn output(&self) -> &Signal<T>;

  fn output_value(&self) -> T {
    self.output().borrow().read()
  }

  fn as_rcrc(self) -> Rc<RefCell<Self>>
  where
    Self: Sized, {
    new_rcrc(self)
  }
}


////////////////////////////////////////////////////////////////////////////////
pub trait BorrowOutputSignal<U: Copy> {
  fn output(&self) -> Signal<U>;
  fn output_value(&self) -> U;
}


////////////////////////////////////////////////////////////////////////////////
impl<T, U> BorrowOutputSignal<U> for Rc<RefCell<T>>
where
  T: HasOutputSignal<U> + ?Sized,
  U: Copy, // Ensure U satisfies Copy
{
  fn output(&self) -> Signal<U> {
    self.borrow().output().clone()
  }

  fn output_value(&self) -> U {
    self.output().read()
  }
}
