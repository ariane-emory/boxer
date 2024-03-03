#![allow(dead_code)]
pub mod cmp;
pub mod counter;
pub mod feedback;
pub mod latch;
pub mod logical;
pub mod math;
pub mod random;
pub mod rcrc;
pub mod sample_and_hold;
pub mod select;
pub mod signal;
pub mod steppable;
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
pub use rcrc::*;
pub use sample_and_hold::*;
pub use select::*;
pub use signal::*;
pub use std::cell::RefCell;
pub use std::rc::Rc;
pub use steppable::*;
pub use timer::*;
pub use trigger::*;
pub use unit_delay::*;
pub use value::*;
pub use waveform::*;

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
