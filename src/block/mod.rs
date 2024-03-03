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
pub type RcRefCell<T> = Rc<RefCell<T>>;


////////////////////////////////////////////////////////////////////////////////
pub type DynSteppableRef = RcRefCell<dyn Steppable>;


///////////////////////////////////////////////////////////////////////////////
pub trait BorrowAndStepSteppable {
  fn step(&self);
}


////////////////////////////////////////////////////////////////////////////////
impl BorrowAndStepSteppable for DynSteppableRef {
  fn step(&self) {
    self.borrow_mut().step();
  }
}


////////////////////////////////////////////////////////////////////////////////
pub fn push_onto_vec_of_rcrc_steppable<T: 'static + Steppable>(
  blocks: &mut Vec<DynSteppableRef>,
  item: &RcRefCell<T>,
) {
  let steppable_item = item.clone() as DynSteppableRef;
  blocks.push(steppable_item);
}


////////////////////////////////////////////////////////////////////////////////
pub struct Signal<T: Copy> {
  value: T,
}
////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Signal<T> {
  pub fn new(value: T) -> Self {
    Signal { value }
  }

  pub fn read(&self) -> T {
    self.value
  }

  pub fn set(&mut self, value: T) {
    self.value = value;
  }
}


////////////////////////////////////////////////////////////////////////////////
pub fn new_rcrc<T>(item: T) -> RcRefCell<T> {
  Rc::new(RefCell::new(item))
}


////////////////////////////////////////////////////////////////////////////////
pub fn new_signal<T: Copy>(value: T) -> SignalRef<T> {
  new_rcrc(Signal::new(value))
}

////////////////////////////////////////////////////////////////////////////////
pub trait HasSignal<T: Copy>: Steppable {
  fn output(&self) -> &SignalRef<T>;

  fn output_value(&self) -> T {
    self.output().borrow().read()
  }

  fn as_rcrc(self) -> RcRefCell<Self>
  where
    Self: Sized,
  {
    new_rcrc(self)
  }
}


////////////////////////////////////////////////////////////////////////////////
pub trait BorrowSignalRef<U: Copy> {
  fn output(&self) -> SignalRef<U>;
  fn output_value(&self) -> U;
}
////////////////////////////////////////////////////////////////////////////////
impl<T, U> BorrowSignalRef<U> for RcRefCell<T>
where
  T: HasSignal<U> + ?Sized,
  U: Copy,
{
  fn output(&self) -> SignalRef<U> {
    self.borrow().output().clone()
  }

  fn output_value(&self) -> U {
    self.output().read()
  }
}


////////////////////////////////////////////////////////////////////////////////
pub type SignalRef<T> = RcRefCell<Signal<T>>;


////////////////////////////////////////////////////////////////////////////////
pub trait BorrowAndReadOrSetSignal<T> {
  fn read(&self) -> T;
  fn set(&self, value: T);
}


////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> BorrowAndReadOrSetSignal<T> for SignalRef<T> {
  fn read(&self) -> T {
    self.borrow().read()
  }

  fn set(&self, value: T) {
    self.borrow_mut().set(value);
  }
}


////////////////////////////////////////////////////////////////////////////////
pub trait BorrowAndSetInputSignal<U: Copy + Default> {
  fn set(&self, input: &SignalRef<U>);
}


////////////////////////////////////////////////////////////////////////////////
impl<U: Copy + Default> BorrowAndSetInputSignal<U> for RcRefCell<Feedback<U>> {
  fn set(&self, input: &SignalRef<U>) {
    self.borrow_mut().set_input(input);
  }
}
