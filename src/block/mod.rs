#![allow(dead_code)]
use rand;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct BlockOutput<T: Copy> {
  value: T,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> BlockOutput<T> {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathAdd<'a, T: std::ops::Add<Output = T> + Copy + Default> {
  pub output: BlockOutput<T>,
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Add<Output = T> + Copy + Default> Block for MathAdd<'a, T> {
  fn step(&mut self) {
    println!("MathAdd::step");
    self.output.set(*self.left.read() + *self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Add<Output = T> + Copy + Default> MathAdd<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    MathAdd {
      output: BlockOutput::new(Default::default()),
      left,
      right,
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathSub<'a, T: std::ops::Sub<Output = T> + Copy + Default> {
  pub output: BlockOutput<T>,
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Sub<Output = T> + Copy + Default> Block for MathSub<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() - *self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Sub<Output = T> + Copy + Default> MathSub<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    MathSub {
      output: BlockOutput::new(Default::default()),
      left,
      right,
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathMul<'a, T: std::ops::Mul<Output = T> + Copy + Default> {
  pub output: BlockOutput<T>,
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Mul<Output = T> + Copy + Default> Block for MathMul<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() * *self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Mul<Output = T> + Copy + Default> MathMul<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    MathMul {
      output: BlockOutput::new(Default::default()),
      left,
      right,
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathDiv<'a, T: std::ops::Div<Output = T> + Copy + Default> {
  pub output: BlockOutput<T>,
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Div<Output = T> + Copy + Default> Block for MathDiv<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() / *self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Div<Output = T> + Copy + Default> MathDiv<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    MathDiv {
      output: BlockOutput::new(Default::default()),
      left,
      right,
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MathMod<'a, T: std::ops::Rem<Output = T> + Copy + Default> {
  pub output: BlockOutput<T>,
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Rem<Output = T> + Copy + Default> Block for MathMod<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() % *self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Rem<Output = T> + Copy + Default> MathMod<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    MathMod {
      output: BlockOutput::new(Default::default()),
      left,
      right,
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Fixed<T: Copy> {
  pub output: BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Fixed<T> {
  pub fn new(value: T) -> Self {
    Fixed {
      output: BlockOutput::new(value),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> Block for Fixed<T> {
  fn step(&mut self) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Select<'a, T: Copy> {
  selector: &'a BlockOutput<bool>,
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
  pub output: BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: Copy> Select<'a, T> {
  pub fn new(
    selector: &'a BlockOutput<bool>,
    left: &'a BlockOutput<T>,
    right: &'a BlockOutput<T>,
  ) -> Self {
    Select {
      selector,
      left,
      right,
      output: BlockOutput::new(*left.read()),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: Copy> Block for Select<'a, T> {
  fn step(&mut self) {
    if *self.selector.read() {
      self.output.set(*self.left.read());
    } else {
      self.output.set(*self.right.read());
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct GreaterThan<'a, T: std::cmp::PartialOrd + Copy> {
  pub output: BlockOutput<bool>,
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::cmp::PartialOrd + Copy> GreaterThan<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    GreaterThan {
      output: BlockOutput::new(false),
      left,
      right,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::cmp::PartialOrd + Copy> Block for GreaterThan<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() > *self.right.read());
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LessThan<'a, T: std::cmp::PartialOrd + Copy> {
  pub output: BlockOutput<bool>,
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::cmp::PartialOrd + Copy> LessThan<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    LessThan {
      output: BlockOutput::new(false),
      left,
      right,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::cmp::PartialOrd + Copy> Block for LessThan<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() < *self.right.read());
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LogicOr<'a> {
  left: &'a BlockOutput<bool>,
  right: &'a BlockOutput<bool>,
  pub output: BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> LogicOr<'a> {
  pub fn new(left: &'a BlockOutput<bool>, right: &'a BlockOutput<bool>) -> Self {
    LogicOr {
      left,
      right,
      output: BlockOutput::new(false),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for LogicOr<'a> {
  fn step(&mut self) {
    self.output.set(*self.left.read() || *self.right.read());
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LogicAnd<'a> {
  left: &'a BlockOutput<bool>,
  right: &'a BlockOutput<bool>,
  pub output: BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> LogicAnd<'a> {
  pub fn new(left: &'a BlockOutput<bool>, right: &'a BlockOutput<bool>) -> Self {
    LogicAnd {
      left,
      right,
      output: BlockOutput::new(false),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for LogicAnd<'a> {
  fn step(&mut self) {
    self.output.set(*self.left.read() && *self.right.read());
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LogicNot<'a> {
  input: &'a BlockOutput<bool>,
  pub output: BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> LogicNot<'a> {
  pub fn new(input: &'a BlockOutput<bool>) -> Self {
    LogicNot {
      input,
      output: BlockOutput::new(false),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for LogicNot<'a> {
  fn step(&mut self) {
    self.output.set(!*self.input.read());
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct LogicXor<'a> {
  pub output: BlockOutput<bool>,
  left: &'a BlockOutput<bool>,
  right: &'a BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> LogicXor<'a> {
  pub fn new(left: &'a BlockOutput<bool>, right: &'a BlockOutput<bool>) -> Self {
    LogicXor {
      left,
      right,
      output: BlockOutput::new(false),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for LogicXor<'a> {
  fn step(&mut self) {
    self.output.set(*self.left.read() ^ *self.right.read());
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct LogicNor<'a> {
  pub output: BlockOutput<bool>,
  left: &'a BlockOutput<bool>,
  right: &'a BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> LogicNor<'a> {
  pub fn new(left: &'a BlockOutput<bool>, right: &'a BlockOutput<bool>) -> Self {
    LogicNor {
      left,
      right,
      output: BlockOutput::new(false),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for LogicNor<'a> {
  fn step(&mut self) {
    self.output.set(!(*self.left.read() || *self.right.read()));
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RiseCounter<'a> {
  input: &'a BlockOutput<bool>,
  max: &'a BlockOutput<usize>,
  count: BlockOutput<usize>,
  at_max: BlockOutput<bool>,
  last_state: bool,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> RiseCounter<'a> {
  fn new(input: &'a BlockOutput<bool>, max: &'a BlockOutput<usize>) -> Self {
    RiseCounter {
      input,
      max,
      count: BlockOutput::new(0),
      at_max: BlockOutput::new(false),
      last_state: false,
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for RiseCounter<'a> {
  fn step(&mut self) {
    let read = *self.input.read();

    if self.count.read() == self.max.read() {
      self.at_max.set(true);
    } else {
      self.at_max.set(false);
    }

    if (!*self.at_max.read()) && *self.input.read() && !self.last_state {
      self.count.set(self.count.read() + 1);
    }

    self.last_state = read;
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RisingTrigger<'a> {
  input: &'a BlockOutput<bool>,
  pub output: BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> RisingTrigger<'a> {
  fn new(input: &'a BlockOutput<bool>) -> Self {
    RisingTrigger {
      input,
      output: BlockOutput::new(false),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for RisingTrigger<'a> {
  fn step(&mut self) {
    let last_state = *self.input.read();
    let input = *self.input.read();

    if input && !last_state {
      self.output.set(true);
    } else {
      self.output.set(false);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct FallingTrigger<'a> {
  input: &'a BlockOutput<bool>,
  pub output: BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> FallingTrigger<'a> {
  fn new(input: &'a BlockOutput<bool>) -> Self {
    FallingTrigger {
      input,
      output: BlockOutput::new(false),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for FallingTrigger<'a> {
  fn step(&mut self) {
    let last_state = *self.input.read();
    let input = *self.input.read();

    if !input && last_state {
      self.output.set(true);
    } else {
      self.output.set(false);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RandomUsize {
  pub output: BlockOutput<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RandomUsize {
  pub fn new() -> Self {
    RandomUsize {
      output: BlockOutput::new(0),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for RandomUsize {
  fn step(&mut self) {
    self.output.set(rand::random());
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-> 'TON' block, which delays a rise by a fixed number of cycles.
pub struct TON<'a> {
  pub output: BlockOutput<bool>,
  pub count: BlockOutput<usize>,
  delay: &'a BlockOutput<usize>,
  reset: &'a BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> TON<'a> {
  pub fn new(delay: &'a BlockOutput<usize>, reset: &'a BlockOutput<bool>) -> Self {
    TON {
      output: BlockOutput::new(false),
      count: BlockOutput::new(0),
      delay,
      reset,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for TON<'a> {
  fn step(&mut self) {
    if *self.reset.read() {
      self.count.set(0);
    } else if *self.output.read() {
      self.count.set(self.count.read() + 1);
    } else {
      self.count.set(0);
    }

    if *self.count.read() >= *self.delay.read() {
      self.output.set(true);
    } else {
      self.output.set(false);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-> 'TOF' block, which delays a fall by a fixed number of cycles.
pub struct TOF<'a> {
  pub output: BlockOutput<bool>,
  pub count: BlockOutput<usize>,
  delay: &'a BlockOutput<usize>,
  reset: &'a BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> TOF<'a> {
  pub fn new(delay: &'a BlockOutput<usize>, reset: &'a BlockOutput<bool>) -> Self {
    TOF {
      output: BlockOutput::new(false),
      count: BlockOutput::new(0),
      delay,
      reset,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for TOF<'a> {
  fn step(&mut self) {
    if *self.reset.read() {
      self.count.set(0);
    } else if !*self.output.read() {
      self.count.set(self.count.read() + 1);
    } else {
      self.count.set(0);
    }

    if *self.count.read() >= *self.delay.read() {
      self.output.set(false);
    } else {
      self.output.set(true);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Basically an IEC 61131-3 'TP' block, which holds it's input for a set number of steps after it rises.
struct TP<'a> {
  pub output: BlockOutput<bool>,
  input: &'a BlockOutput<bool>,
  count_from: &'a BlockOutput<usize>,
  count: BlockOutput<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> TP<'a> {
  pub fn new(input: &'a BlockOutput<bool>, count_from: &'a BlockOutput<usize>) -> Self {
    TP {
      output: BlockOutput::new(false),
      input,
      count_from,
      count: BlockOutput::new(0),
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for TP<'a> {
  fn step(&mut self) {
    if *self.input.read() {
      self.count.set(*self.count_from.read());
    }

    if *self.count.read() > 0usize {
      self.output.set(true);
      self.count.set(self.count.read() - 1);
    } else {
      self.output.set(false);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct SRLatch<'a> {
  pub output: BlockOutput<bool>,
  set: &'a BlockOutput<bool>,
  reset: &'a BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> SRLatch<'a> {
  pub fn new(set: &'a BlockOutput<bool>, reset: &'a BlockOutput<bool>) -> Self {
    SRLatch {
      output: BlockOutput::new(false),
      set,
      reset,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for SRLatch<'a> {
  fn step(&mut self) {
    if *self.set.read() {
      self.output.set(true);
    } else if *self.reset.read() {
      self.output.set(false);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct RSLatch<'a> {
  pub output: BlockOutput<bool>,
  set: &'a BlockOutput<bool>,
  reset: &'a BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> RSLatch<'a> {
  pub fn new(set: &'a BlockOutput<bool>, reset: &'a BlockOutput<bool>) -> Self {
    RSLatch {
      output: BlockOutput::new(false),
      set,
      reset,
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for RSLatch<'a> {
  fn step(&mut self) {
    if *self.reset.read() {
      self.output.set(false);
    } else if *self.set.read() {
      self.output.set(true);
    }
  }
}
