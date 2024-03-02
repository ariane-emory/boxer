////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size {
  pub height: usize,
  pub width: usize,
}

////////////////////////////////////////////////////////////////////////////////
impl Size {
  pub fn new(
    height: usize,
    width: usize,
  ) -> Size {
    Size { height, width }
  }

  pub fn area(&self) -> usize {
    if self.height == 1 && self.width == 1 {
      1
    } else if self.height == 1 {
      self.width
    } else if self.width == 1 {
      self.height
    } else {
      self.height * self.width
    }
  }

  pub fn is_tall(&self) -> bool {
    self.height > self.width
  }

  pub fn is_wide(&self) -> bool {
    self.height < self.width
  }
}
