use crate::block::*;


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
pub type SignalRef<T> = RcRefCell<Signal<T>>;


////////////////////////////////////////////////////////////////////////////////
pub fn new_signal_ref<T: Copy>(value: T) -> SignalRef<T> {
  new_rcrc(Signal::new(value))
}


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
