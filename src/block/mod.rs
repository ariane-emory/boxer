#![allow(dead_code)]
#![allow(unused_imports)]
//use rand;
use std::cell::RefCell;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Block {
  fn step(&mut self);
}


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
type Rcrcbo<T> = Rc<RefCell<BlockOutput<T>>>;


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Value<T: Copy> {
  pub output: Rcrcbo<T>,
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
impl<T: Copy> Block for Value<T> {
  fn step(&mut self) {}
}



////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathAdd<T: std::ops::Add<Output = T> + Copy + Default> {
  pub output: Rcrcbo<T>,
  left: Rcrcbo<T>,
  right: Rcrcbo<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Add<Output = T> + Copy + Default> Block for MathAdd<T> {
  fn step(&mut self) {
    println!("MathAdd::step");
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() + *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Add<Output = T> + Copy + Default> MathAdd<T> {
  pub fn new(left: &Rcrcbo<T>, right: &Rcrcbo<T>) -> Self {
    MathAdd {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathSub<T: std::ops::Sub<Output = T> + Copy + Default> {
  pub output: Rcrcbo<T>,
  left: Rcrcbo<T>,
  right: Rcrcbo<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Sub<Output = T> + Copy + Default> Block for MathSub<T> {
  fn step(&mut self) {
    println!("MathSub::step");
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() - *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Sub<Output = T> + Copy + Default> MathSub<T> {
  pub fn new(left: &Rcrcbo<T>, right: &Rcrcbo<T>) -> Self {
    MathSub {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathMul<T: std::ops::Mul<Output = T> + Copy + Default> {
  pub output: Rcrcbo<T>,
  left: Rcrcbo<T>,
  right: Rcrcbo<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> Block for MathMul<T> {
  fn step(&mut self) {
    println!("MathMul::step");
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() * *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Mul<Output = T> + Copy + Default> MathMul<T> {
  pub fn new(left: &Rcrcbo<T>, right: &Rcrcbo<T>) -> Self {
    MathMul {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathDiv<T: std::ops::Div<Output = T> + Copy + Default> {
  pub output: Rcrcbo<T>,
  left: Rcrcbo<T>,
  right: Rcrcbo<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> Block for MathDiv<T> {
  fn step(&mut self) {
    println!("MathDiv::step");
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() / *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Div<Output = T> + Copy + Default> MathDiv<T> {
  pub fn new(left: &Rcrcbo<T>, right: &Rcrcbo<T>) -> Self {
    MathDiv {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathMod<T: std::ops::Rem<Output = T> + Copy + Default> {
  pub output: Rcrcbo<T>,
  left: Rcrcbo<T>,
  right: Rcrcbo<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Rem<Output = T> + Copy + Default> Block for MathMod<T> {
  fn step(&mut self) {
    println!("MathMod::step");
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() % *self.right.borrow().read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::ops::Rem<Output = T> + Copy + Default> MathMod<T> {
  pub fn new(left: &Rcrcbo<T>, right: &Rcrcbo<T>) -> Self {
    MathMod {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Select<T: Copy> {
  which: Rcrcbo<bool>,
  left: Rcrcbo<T>,
  right: Rcrcbo<T>,
  pub output: BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Select<T> {
  pub fn new(which: &Rcrcbo<bool>, left: &Rcrcbo<T>, right: &Rcrcbo<T>) -> Self {
    Select {
      which: Rc::clone(which),
      left: Rc::clone(left),
      right: Rc::clone(right),
      output: BlockOutput::new(*left.borrow().read()),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Block for Select<T> {
  fn step(&mut self) {
    if *self.which.borrow().read() {
      self.output.set(*self.left.borrow().read());
    } else {
      self.output.set(*self.right.borrow().read());
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
fn rcrcbo_new<T: Copy>(value: T) -> Rcrcbo<T> {
  Rc::new(RefCell::new(BlockOutput::new(value)))
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct GreaterThan<T: std::cmp::PartialOrd + Copy> {
  pub output: Rcrcbo<bool>,
  left: Rcrcbo<T>,
  right: Rcrcbo<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> GreaterThan<T> {
  pub fn new(left: &Rcrcbo<T>, right: &Rcrcbo<T>) -> Self {
    GreaterThan {
      output: rcrcbo_new(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> Block for GreaterThan<T> {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() > *self.right.borrow().read());
  }
}


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// pub struct LessThan<T: std::cmp::PartialOrd + Copy> {
//   pub output: BlockOutput<bool>,
//   left: Rcrcbo<T>,
//   right: Rcrcbo<T>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<T: std::cmp::PartialOrd + Copy> LessThan<T> {
//   pub fn new(left: Rcrcbo<T>, right: Rcrcbo<T>) -> Self {
//     LessThan {
//       output: BlockOutput::new(false),
//       left,
//       right,
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<T: std::cmp::PartialOrd + Copy> Block for LessThan<T> {
//   fn step(&mut self) {
//     self.output.set(*self.left.borrow().read() < *self.right.borrow().read());
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// pub struct LogicOr<'a> {
//   left: Rcrcbo<bool>,
//   right: Rcrcbo<bool>,
//   pub output: BlockOutput<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> LogicOr<'a> {
//   pub fn new(left: Rcrcbo<bool>, right: Rcrcbo<bool>) -> Self {
//     LogicOr {
//       left,
//       right,
//       output: BlockOutput::new(false),
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for LogicOr<'a> {
//   fn step(&mut self) {
//     self.output.set(*self.left.borrow().read() || *self.right.borrow().read());
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// pub struct LogicAnd<'a> {
//   left: Rcrcbo<bool>,
//   right: Rcrcbo<bool>,
//   pub output: BlockOutput<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> LogicAnd<'a> {
//   pub fn new(left: Rcrcbo<bool>, right: Rcrcbo<bool>) -> Self {
//     LogicAnd {
//       left,
//       right,
//       output: BlockOutput::new(false),
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for LogicAnd<'a> {
//   fn step(&mut self) {
//     self.output.set(*self.left.borrow().read() && *self.right.borrow().read());
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// pub struct LogicNot<'a> {
//   input: Rcrcbo<bool>,
//   pub output: BlockOutput<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> LogicNot<'a> {
//   pub fn new(input: Rcrcbo<bool>) -> Self {
//     LogicNot {
//       input,
//       output: BlockOutput::new(false),
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for LogicNot<'a> {
//   fn step(&mut self) {
//     self.output.set(!*self.input.borrow().read());
//   }
// }

// ////////////////////////////////////////////////////////////////////////////////////////////////////
// struct LogicXor<'a> {
//   pub output: BlockOutput<bool>,
//   left: Rcrcbo<bool>,
//   right: Rcrcbo<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> LogicXor<'a> {
//   pub fn new(left: Rcrcbo<bool>, right: Rcrcbo<bool>) -> Self {
//     LogicXor {
//       left,
//       right,
//       output: BlockOutput::new(false),
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for LogicXor<'a> {
//   fn step(&mut self) {
//     self.output.set(*self.left.borrow().read() ^ *self.right.borrow().read());
//   }
// }

// ////////////////////////////////////////////////////////////////////////////////////////////////////
// struct LogicNor<'a> {
//   pub output: BlockOutput<bool>,
//   left: Rcrcbo<bool>,
//   right: Rcrcbo<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> LogicNor<'a> {
//   pub fn new(left: Rcrcbo<bool>, right: Rcrcbo<bool>) -> Self {
//     LogicNor {
//       left,
//       right,
//       output: BlockOutput::new(false),
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for LogicNor<'a> {
//   fn step(&mut self) {
//     self.output.set(!(*self.left.borrow().read() || *self.right.borrow().read()));
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// pub struct RiseCounter<'a> {
//   pub count: BlockOutput<usize>,
//   pub at_max: BlockOutput<bool>,
//   input: Rcrcbo<bool>,
//   max: &'a BlockOutput<usize>,
//   last_state: bool,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> RiseCounter<'a> {
//   pub fn new(input: Rcrcbo<bool>, max: &'a BlockOutput<usize>) -> Self {
//     RiseCounter {
//       count: BlockOutput::new(0),
//       at_max: BlockOutput::new(false),
//       input,
//       max,
//       last_state: false,
//     }
//   }
// }

// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for RiseCounter<'a> {
//   fn step(&mut self) {
//     let read = *self.input.borrow().read();

//     if self.count.borrow().read() == self.max.borrow().read() {
//       self.at_max.set(true);
//     } else {
//       self.at_max.set(false);
//     }

//     if (!*self.at_max.borrow().read()) && *self.input.borrow().read() && !self.last_state {
//       self.count.set(self.count.borrow().read() + 1);
//     }

//     self.last_state = read;
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// pub struct RisingTrigger<'a> {
//   input: Rcrcbo<bool>,
//   pub output: BlockOutput<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> RisingTrigger<'a> {
//   pub fn new(input: Rcrcbo<bool>) -> Self {
//     RisingTrigger {
//       input,
//       output: BlockOutput::new(false),
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for RisingTrigger<'a> {
//   fn step(&mut self) {
//     let last_state = *self.input.borrow().read();
//     let input = *self.input.borrow().read();

//     if input && !last_state {
//       self.output.set(true);
//     } else {
//       self.output.set(false);
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// pub struct FallingTrigger<'a> {
//   input: Rcrcbo<bool>,
//   pub output: BlockOutput<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> FallingTrigger<'a> {
//   pub fn new(input: Rcrcbo<bool>) -> Self {
//     FallingTrigger {
//       input,
//       output: BlockOutput::new(false),
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for FallingTrigger<'a> {
//   fn step(&mut self) {
//     let last_state = *self.input.borrow().read();
//     let input = *self.input.borrow().read();

//     if !input && last_state {
//       self.output.set(true);
//     } else {
//       self.output.set(false);
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// pub struct RandomUsize {
//   pub output: BlockOutput<usize>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl RandomUsize {
//   pub fn new() -> Self {
//     RandomUsize {
//       output: BlockOutput::new(0),
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl Block for RandomUsize {
//   fn step(&mut self) {
//     self.output.set(rand::random());
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// // Basically an IEC 61131-> 'TON' block, which delays a rise by a fixed number of cycles.
// pub struct TON<'a> {
//   pub output: BlockOutput<bool>,
//   pub count: BlockOutput<usize>,
//   delay: &'a BlockOutput<usize>,
//   reset: Rcrcbo<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> TON<'a> {
//   pub fn new(delay: &'a BlockOutput<usize>, reset: Rcrcbo<bool>) -> Self {
//     TON {
//       output: BlockOutput::new(false),
//       count: BlockOutput::new(0),
//       delay,
//       reset,
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for TON<'a> {
//   fn step(&mut self) {
//     if *self.reset.borrow().read() {
//       self.count.set(0);
//     } else if *self.output.borrow().read() {
//       self.count.set(self.count.borrow().read() + 1);
//     } else {
//       self.count.set(0);
//     }

//     if *self.count.borrow().read() >= *self.delay.borrow().read() {
//       self.output.set(true);
//     } else {
//       self.output.set(false);
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// // Basically an IEC 61131-> 'TOF' block, which delays a fall by a fixed number of cycles.
// pub struct TOF<'a> {
//   pub output: BlockOutput<bool>,
//   pub count: BlockOutput<usize>,
//   delay: &'a BlockOutput<usize>,
//   reset: Rcrcbo<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> TOF<'a> {
//   pub fn new(delay: &'a BlockOutput<usize>, reset: Rcrcbo<bool>) -> Self {
//     TOF {
//       output: BlockOutput::new(false),
//       count: BlockOutput::new(0),
//       delay,
//       reset,
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for TOF<'a> {
//   fn step(&mut self) {
//     if *self.reset.borrow().read() {
//       self.count.set(0);
//     } else if !*self.output.borrow().read() {
//       self.count.set(self.count.borrow().read() + 1);
//     } else {
//       self.count.set(0);
//     }

//     if *self.count.borrow().read() >= *self.delay.borrow().read() {
//       self.output.set(false);
//     } else {
//       self.output.set(true);
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// // Basically an IEC 61131-3 'TP' block, which holds it's input for a set number of steps after it rises.
// struct TP<'a> {
//   pub output: BlockOutput<bool>,
//   input: Rcrcbo<bool>,
//   count_from: &'a BlockOutput<usize>,
//   count: BlockOutput<usize>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> TP<'a> {
//   pub fn new(input: Rcrcbo<bool>, count_from: &'a BlockOutput<usize>) -> Self {
//     TP {
//       output: BlockOutput::new(false),
//       input,
//       count_from,
//       count: BlockOutput::new(0),
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for TP<'a> {
//   fn step(&mut self) {
//     if *self.input.borrow().read() {
//       self.count.set(*self.count_from.borrow().read());
//     }

//     if *self.count.borrow().read() > 0usize {
//       self.output.set(true);
//       self.count.set(self.count.borrow().read() - 1);
//     } else {
//       self.output.set(false);
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// struct SRLatch<'a> {
//   pub output: BlockOutput<bool>,
//   set: Rcrcbo<bool>,
//   reset: Rcrcbo<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> SRLatch<'a> {
//   pub fn new(set: Rcrcbo<bool>, reset: Rcrcbo<bool>) -> Self {
//     SRLatch {
//       output: BlockOutput::new(false),
//       set,
//       reset,
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for SRLatch<'a> {
//   fn step(&mut self) {
//     if *self.set.borrow().read() {
//       self.output.set(true);
//     } else if *self.reset.borrow().read() {
//       self.output.set(false);
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// struct RSLatch<'a> {
//   pub output: BlockOutput<bool>,
//   set: Rcrcbo<bool>,
//   reset: Rcrcbo<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> RSLatch<'a> {
//   pub fn new(set: Rcrcbo<bool>, reset: Rcrcbo<bool>) -> Self {
//     RSLatch {
//       output: BlockOutput::new(false),
//       set,
//       reset,
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl<'a> Block for RSLatch<'a> {
//   fn step(&mut self) {
//     if *self.reset.borrow().read() {
//       self.output.set(false);
//     } else if *self.set.borrow().read() {
//       self.output.set(true);
//     }
//   }
// }
