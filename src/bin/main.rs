// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::process_file::*;
use boxer::simple_geo::find_rectangles;
use boxer::simple_geo::ConnectedLine;
use boxer::simple_geo::Offsetable;
use boxer::simple_geo::Orientation;
use boxer::simple_geo::Orientation::*;
use boxer::simple_geo::Point;
use boxer::simple_geo::Word;
use std::io::{self};

////////////////////////////////////////////////////////////////////////////////////////////////////
static LINE_OFFSET: isize = 1;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  //loop {
  let filename = "./data/simple.box";
  let (_, _, _, _) = process_file(filename);

  Ok(())
}
