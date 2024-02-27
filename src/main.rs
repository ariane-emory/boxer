// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
// use util::*;
mod process_file;
mod simple_geo;
#[macro_use]
mod util;
mod simple_matrix;

use process_file::process_file;
//use simple_geo::*;
//use simple_matrix::*;

use std::io::{self};

type Point = simple_geo::Point;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  let _ = process_file(
    "./data/data.txt",
    Box::new(|pos: &Point, byte: &u8| {
      if 0 != (*byte & 128) {
        panic!("Found non-ASCII byte {} at {:?}", byte, pos);
      }

      println!("Horiz {:?}: '{}'", pos, *byte as char);
    }),
    Box::new(|pos: &Point, byte: &u8| {
      // Print an inverted posittion to reflext the character's original posiion in the file:
      let inverted_pos = Point::new(pos.line, pos.col);
      println!("Vert  {:?}: '{}'", inverted_pos, *byte as char);
    }),
  );

  Ok(())
}
