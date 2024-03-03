use crate::block::*;


////////////////////////////////////////////////////////////////////////////////
pub trait Steppable {
  fn step(&mut self);
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
pub trait BorrowSignalRefAndGetOutput<U: Copy> {
  fn output(&self) -> SignalRef<U>;
  fn output_value(&self) -> U {
    self.output().read()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T, U> BorrowSignalRefAndGetOutput<U> for RcRefCell<T>
where
  T: HasSignal<U>,
  U: Copy,
{
  fn output(&self) -> SignalRef<U> {
    self.borrow().output().clone()
  }
}


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
