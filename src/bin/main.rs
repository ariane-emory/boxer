// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::process_file::*;
use std::io::{self};

////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  process_file("./data/complex.box")?;
  Ok(())
}
