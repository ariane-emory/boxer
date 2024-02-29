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

////////////////////////////////////////////////////////////////////////////////////////////////////
struct Adder<'a, T> {
  left_addend: &'a BlockOutput<T>,
  right_addend: &'a BlockOutput<T>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a, T> Adder<'a, T> {
  pub fn new(left_addend: &'a BlockOutput<T>, right_addend: &'a BlockOutput<T>) -> Self {
    Adder {
      left_addend,
      right_addend,
    }
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
