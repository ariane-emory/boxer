use crate::block::*;


////////////////////////////////////////////////////////////////////////////////
pub trait Steppable {
  fn step(&mut self);

  fn as_rcrc(self) -> RcRefCell<Self>
  where
    Self: Sized,
  {
    new_rcrc(self)
  }
}


////////////////////////////////////////////////////////////////////////////////
pub trait HasOutputSignal<T: Copy> {
  fn output(&self) -> &SignalRef<T>;

  fn output_value(&self) -> T {
    self.output().borrow().read()
  }
}


////////////////////////////////////////////////////////////////////////////////
pub trait BorrowSteppableRefAndGetOutput<U: Copy> {
  fn output(&self) -> SignalRef<U>;

  fn output_value(&self) -> U {
    self.output().read()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl<T, U> BorrowSteppableRefAndGetOutput<U> for RcRefCell<T>
where
  T: HasOutputSignal<U>,
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
