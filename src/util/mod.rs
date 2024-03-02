mod errstring;
mod max_line_len;
mod noisy_println;

use std::cell::RefCell;
use std::rc::Rc;

pub use errstring::*;
pub use max_line_len::*;
pub use noisy_println::*;

////////////////////////////////////////////////////////////////////////////////
pub fn new_rcrc<T>(item: T) -> Rc<RefCell<T>> {
  Rc::new(RefCell::new(item))
}
