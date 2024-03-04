// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::process_file::*;
use std::io::{self};

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  //loop {
  let filename = "./data/simple.box";
  let (_, _, _, _) = process_file(filename)?;

  Ok(())
}
