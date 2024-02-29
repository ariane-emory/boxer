#![allow(dead_code)]

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct BlockOutput<T> {
  value: T,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> BlockOutput<T> {
  pub fn read(&self) -> &T {
    &self.value
  }

  pub fn set(&mut self, value: T) {
    self.value = value;
  }

  pub fn new(value: T) -> Self {
    BlockOutput { value }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Block {
  fn step(&mut self);
}

// ////////////////////////////////////////////////////////////////////////////////////////////////////
// struct Adder<'a, T> {
//   left_addend: &'a BlockOutput<T>,
//   right_addend: &'a BlockOutput<T>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a, T> Adder<'a, T> {
//   pub fn new(left_addend: &'a dyn BlockOutput<T>, right_addend: &'a dyn BlockOutput<T>) -> Self {
//     Adder {
//       left_addend,
//       right_addend,
//     }
//   }
// }

// ////////////////////////////////////////////////////////////////////////////////////////////////////
// struct Counter<'a> {
//   source: &'a dyn BlockOutput<bool>,
//   count: usize,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Counter<'a> {
//   fn new(source: &'a impl BlockOutput<bool>) -> Self {
//     Counter { source, count: 0 }
//   }
// }

// ////////////////////////////////////////////////////////////////////////////////////////////////////
// struct RisingTrigger<'a> {
//   source: &'a dyn BlockOutput<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> RisingTrigger<'a> {
//   fn new(source: &'a impl BlockOutput<bool>) -> Self {
//     RisingTrigger { source }
//   }
// }
