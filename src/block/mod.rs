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
type Signal<T> = Rc<RefCell<BlockOutput<T>>>;
////////////////////////////////////////////////////////////////////////////////////////////////////
fn new_signal<T: Copy>(value: T) -> Signal<T> {
  Rc::new(RefCell::new(BlockOutput::new(value)))
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Value<T: Copy> {
  pub output: Signal<T>,
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
  pub output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
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
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    MathAdd {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathSub<T: std::ops::Sub<Output = T> + Copy + Default> {
  pub output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
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
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    MathSub {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathMul<T: std::ops::Mul<Output = T> + Copy + Default> {
  pub output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
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
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    MathMul {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathDiv<T: std::ops::Div<Output = T> + Copy + Default> {
  pub output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
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
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    MathDiv {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathMod<T: std::ops::Rem<Output = T> + Copy + Default> {
  pub output: Signal<T>,
  left: Signal<T>,
  right: Signal<T>,
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
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    MathMod {
      output: Rc::new(RefCell::new(BlockOutput::new(Default::default()))),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Select<T: Copy> {
  which: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
  pub output: BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Select<T> {
  pub fn new(which: &Signal<bool>, left: &Signal<T>, right: &Signal<T>) -> Self {
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
pub struct GreaterThan<T: std::cmp::PartialOrd + Copy> {
  pub output: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> GreaterThan<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    GreaterThan {
      output: new_signal(false),
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


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LessThan<T: std::cmp::PartialOrd + Copy> {
  pub output: Signal<bool>,
  left: Signal<T>,
  right: Signal<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> LessThan<T> {
  pub fn new(left: &Signal<T>, right: &Signal<T>) -> Self {
    LessThan {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: std::cmp::PartialOrd + Copy> Block for LessThan<T> {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() < *self.right.borrow().read());
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LogicOr {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicOr {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicOr {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for LogicOr {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() || *self.right.borrow().read());
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LogicAnd {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicAnd {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicAnd {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for LogicAnd {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() && *self.right.borrow().read());
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LogicNot {
  pub output: Signal<bool>,
  input: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicNot {
  pub fn new(input: &Signal<bool>) -> Self {
    LogicNot {
      output: new_signal(false),
      input: Rc::clone(input),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for LogicNot {
  fn step(&mut self) {
    self.output.borrow_mut().set(!*self.input.borrow().read());
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
struct LogicXor {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicXor {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicXor {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for LogicXor {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(*self.left.borrow().read() ^ *self.right.borrow().read());
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
struct LogicNor {
  pub output: Signal<bool>,
  left: Signal<bool>,
  right: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LogicNor {
  pub fn new(left: &Signal<bool>, right: &Signal<bool>) -> Self {
    LogicNor {
      output: new_signal(false),
      left: Rc::clone(left),
      right: Rc::clone(right),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for LogicNor {
  fn step(&mut self) {
    self
      .output
      .borrow_mut()
      .set(!(*self.left.borrow().read() || *self.right.borrow().read()));
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RiseCounter {
  pub count: Signal<usize>,
  pub at_max: Signal<bool>,
  input: Signal<bool>,
  max: Signal<usize>,
  last_state: bool,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RiseCounter {
  pub fn new(input: &Signal<bool>, max: &Signal<usize>) -> Self {
    RiseCounter {
      count: new_signal(0),
      at_max: new_signal(false),
      input: Rc::clone(input),
      max: Rc::clone(max),
      last_state: false,
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RisingTrigger {
  pub output: Signal<bool>,
  input: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RisingTrigger {
  pub fn new(input: &Signal<bool>) -> Self {
    RisingTrigger {
      output: new_signal(false),
      input: Rc::clone(input),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for RisingTrigger {
  fn step(&mut self) {
    let last_state = *self.input.borrow().read();
    let input = *self.input.borrow().read();

    if input && !last_state {
      self.output.borrow_mut().set(true);
    } else {
      self.output.borrow_mut().set(false);
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct FallingTrigger {
  pub output: Signal<bool>,
  input: Signal<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FallingTrigger {
  pub fn new(input: &Signal<bool>) -> Self {
    FallingTrigger {
      output: new_signal(false),
      input: Rc::clone(input),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for FallingTrigger {
  fn step(&mut self) {
    let last_state = *self.input.borrow().read();
    let input = *self.input.borrow().read();

    if !input && last_state {
      self.output.borrow_mut().set(true);
    } else {
      self.output.borrow_mut().set(false);
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RandomUsize {
  pub output: Signal<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RandomUsize {
  pub fn new() -> Self {
    RandomUsize {
      output: new_signal(0),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for RandomUsize {
  fn step(&mut self) {
    self.output.borrow_mut().set(rand::random());
  }
}


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// // Basically an IEC 61131-> 'TON' block, which delays a rise by a fixed number of cycles.
// pub struct TON {
//   pub output: Signal<bool>,
//   pub count: BlockOutput<usize>,
//   delay: &'a BlockOutput<usize>,
//   reset: Signal<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl TON {
//   pub fn new(delay: &'a BlockOutput<usize>, reset: Signal<bool>) -> Self {
//     TON {
//       output: new_signal(false),
//       count: new_signal(0),
//       delay,
//       reset,
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl Block for TON {
//   fn step(&mut self) {
//     if *self.reset.borrow().read() {
//       self.count.set(0);
//     } else if *self.output.borrow().read() {
//       self.count.set(self.count.borrow().read() + 1);
//     } else {
//       self.count.set(0);
//     }

//     if *self.count.borrow().read() >= *self.delay.borrow().read() {
//       self.output.borrow_mut().set(true);
//     } else {
//       self.output.borrow_mut().set(false);
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// // Basically an IEC 61131-> 'TOF' block, which delays a fall by a fixed number of cycles.
// pub struct TOF {
//   pub output: Signal<bool>,
//   pub count: BlockOutput<usize>,
//   delay: &'a BlockOutput<usize>,
//   reset: Signal<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl TOF {
//   pub fn new(delay: &'a BlockOutput<usize>, reset: Signal<bool>) -> Self {
//     TOF {
//       output: new_signal(false),
//       count: new_signal(0),
//       delay,
//       reset,
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl Block for TOF {
//   fn step(&mut self) {
//     if *self.reset.borrow().read() {
//       self.count.set(0);
//     } else if !*self.output.borrow().read() {
//       self.count.set(self.count.borrow().read() + 1);
//     } else {
//       self.count.set(0);
//     }

//     if *self.count.borrow().read() >= *self.delay.borrow().read() {
//       self.output.borrow_mut().set(false);
//     } else {
//       self.output.borrow_mut().set(true);
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// // Basically an IEC 61131-3 'TP' block, which holds it's input for a set number of steps after it rises.
// struct TP {
//   pub output: Signal<bool>,
//   input: Signal<bool>,
//   count_from: &'a BlockOutput<usize>,
//   count: BlockOutput<usize>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl TP {
//   pub fn new(input: Signal<bool>, count_from: &'a BlockOutput<usize>) -> Self {
//     TP {
//       output: new_signal(false),
//       input,
//       count_from,
//       count: new_signal(0),
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl Block for TP {
//   fn step(&mut self) {
//     if *self.input.borrow().read() {
//       self.count.set(*self.count_from.borrow().read());
//     }

//     if *self.count.borrow().read() > 0usize {
//       self.output.borrow_mut().set(true);
//       self.count.set(self.count.borrow().read() - 1);
//     } else {
//       self.output.borrow_mut().set(false);
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// struct SRLatch {
//   pub output: Signal<bool>,
//   set: Signal<bool>,
//   reset: Signal<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl SRLatch {
//   pub fn new(set: Signal<bool>, reset: Signal<bool>) -> Self {
//     SRLatch {
//       output: new_signal(false),
//       set,
//       reset,
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl Block for SRLatch {
//   fn step(&mut self) {
//     if *self.set.borrow().read() {
//       self.output.borrow_mut().set(true);
//     } else if *self.reset.borrow().read() {
//       self.output.borrow_mut().set(false);
//     }
//   }
// }


// ////////////////////////////////////////////////////////////////////////////////////////////////////
// struct RSLatch {
//   pub output: Signal<bool>,
//   set: Signal<bool>,
//   reset: Signal<bool>,
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl RSLatch {
//   pub fn new(set: Signal<bool>, reset: Signal<bool>) -> Self {
//     RSLatch {
//       output: new_signal(false),
//       set,
//       reset,
//     }
//   }
// }
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// impl Block for RSLatch {
//   fn step(&mut self) {
//     if *self.reset.borrow().read() {
//       self.output.borrow_mut().set(false);
//     } else if *self.set.borrow().read() {
//       self.output.borrow_mut().set(true);
//     }
//   }
// }
