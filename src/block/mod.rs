#![allow(dead_code)]
use rand;

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

////////////////////////////////////////////////////////////////////////////////////////////////////
struct MathAdder<'a, T> {
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
  output: BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Add<Output = T> + Copy> Block for MathAdder<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() + *self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Add<Output = T> + Copy> MathAdder<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    MathAdder {
      left,
      right,
      output: BlockOutput::new(*left.read() + *right.read()),
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct MathSubtractor<'a, T> {
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
  output: BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Sub<Output = T> + Copy> Block for MathSubtractor<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() - *self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Sub<Output = T> + Copy> MathSubtractor<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    MathSubtractor {
      left,
      right,
      output: BlockOutput::new(*left.read() - *right.read()),
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct MathMultiplier<'a, T> {
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
  output: BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Mul<Output = T> + Copy> Block for MathMultiplier<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() * *self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Mul<Output = T> + Copy> MathMultiplier<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    MathMultiplier {
      left,
      right,
      output: BlockOutput::new(*left.read() * *right.read()),
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct MathDiv<'a, T> {
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
  output: BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Div<Output = T> + Copy> Block for MathDiv<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() / *self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Div<Output = T> + Copy> MathDiv<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    MathDiv {
      left,
      right,
      output: BlockOutput::new(*left.read() / *right.read()),
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct MathMod<'a, T> {
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
  output: BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Rem<Output = T> + Copy> Block for MathMod<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() % *self.right.read());
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::ops::Rem<Output = T> + Copy> MathMod<'a, T> {
  pub fn new(left: &'a BlockOutput<T>, right: &'a BlockOutput<T>) -> Self {
    MathMod {
      left,
      right,
      output: BlockOutput::new(*left.read() % *right.read()),
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct FixedValue<T> {
  value: T,
  output: BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> FixedValue<T> {
  pub fn new(value: T) -> Self {
    FixedValue {
      value,
      output: BlockOutput::new(value),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Block for FixedValue<T> {
  fn step(&mut self) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct SelectValue<'a, T> {
  selector: &'a BlockOutput<bool>,
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
  output: BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: Copy> SelectValue<'a, T> {
  pub fn new(
    selector: &'a BlockOutput<bool>,
    left: &'a BlockOutput<T>,
    right: &'a BlockOutput<T>,
  ) -> Self {
    SelectValue {
      selector,
      left,
      right,
      output: BlockOutput::new(*left.read()),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: Clone> Block for SelectValue<'a, T> {
  fn step(&mut self) {
    if *self.selector.read() {
      self.output.set(self.left.read().clone());
    } else {
      self.output.set(self.right.read().clone());
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct GreaterThan<'a, T> {
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
  output: BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::cmp::PartialOrd> Block for GreaterThan<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() > *self.right.read());
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct LessThan<'a, T> {
  left: &'a BlockOutput<T>,
  right: &'a BlockOutput<T>,
  output: BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T: std::cmp::PartialOrd> Block for LessThan<'a, T> {
  fn step(&mut self) {
    self.output.set(*self.left.read() < *self.right.read());
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct LogicOr<'a> {
  left: &'a BlockOutput<bool>,
  right: &'a BlockOutput<bool>,
  output: BlockOutput<bool>,
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
struct LogicAnd<'a> {
  left: &'a BlockOutput<bool>,
  right: &'a BlockOutput<bool>,
  output: BlockOutput<bool>,
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
struct LogicNot<'a> {
  input: &'a BlockOutput<bool>,
  output: BlockOutput<bool>,
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
struct RiseCounter<'a> {
  source: &'a BlockOutput<bool>,
  last_state: bool,
  count: BlockOutput<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> RiseCounter<'a> {
  fn new(source: &'a BlockOutput<bool>) -> Self {
    RiseCounter {
      source,
      last_state: false,
      count: BlockOutput::new(0),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> Block for RiseCounter<'a> {
  fn step(&mut self) {
    let read = *self.source.read();

    if read && !self.last_state {
      self.count.set(self.count.read() + 1);
    }

    self.last_state = read;
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct RisingTrigger<'a> {
  source: &'a BlockOutput<bool>,
  output: BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> RisingTrigger<'a> {
  fn new(source: &'a BlockOutput<bool>) -> Self {
    RisingTrigger {
      source,
      output: BlockOutput::new(false),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for RisingTrigger<'_> {
  fn step(&mut self) {
    let last_state = *self.source.read();
    let input = *self.source.read();

    if input && !last_state {
      self.output.set(true);
    } else {
      self.output.set(false);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct FallingTrigger<'a> {
  source: &'a BlockOutput<bool>,
  output: BlockOutput<bool>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> FallingTrigger<'a> {
  fn new(source: &'a BlockOutput<bool>) -> Self {
    FallingTrigger {
      source,
      output: BlockOutput::new(false),
    }
  }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for FallingTrigger<'_> {
  fn step(&mut self) {
    let last_state = *self.source.read();
    let input = *self.source.read();

    if !input && last_state {
      self.output.set(true);
    } else {
      self.output.set(false);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct RandomUsize {
  output: BlockOutput<usize>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Block for RandomUsize {
  fn step(&mut self) {
    self.output.set(rand::random());
  }
}
