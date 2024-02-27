#![allow(unused_imports)]
#![allow(unused_variables)]
// #![allow(unused_mut)]
#![allow(dead_code)]
mod line_maker;
mod process_file;
mod simple_geo;
#[macro_use]
mod util;
mod simple_matrix;
use crate::simple_geo::find_rectangles;
use line_maker::LineMaker;
use process_file::process_file;
use simple_geo::{Line, Point};
use std::cell::RefCell;
use std::io::{self};
use std::rc::Rc;
////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> io::Result<()> {
  let horiz_lm = Rc::new(RefCell::new(LineMaker::new(b'-')));
  let horiz_lm_clone = Rc::clone(&horiz_lm);

  let process_horiz = Box::new(move |pos: &Point, byte: &u8| {
    if 0 != (*byte & 128) {
      panic!("Found non-ASCII byte {} at {:?}", byte, pos);
    }

    let mut lm = horiz_lm_clone.borrow_mut();
    lm.process(pos, byte);

    println!("Horiz {:?}: '{}'", pos, *byte as char);
  });

  let vert_lm = Rc::new(RefCell::new(LineMaker::new(b'|')));
  let vert_lm_clone = Rc::clone(&vert_lm);

  let process_vert = Box::new(move |pos: &Point, byte: &u8| {
    let inverted_pos = Point::new(pos.line, pos.col);

    let mut lm = vert_lm_clone.borrow_mut();
    lm.process(&inverted_pos, byte);

    println!("Vert  {:?}: '{}'", inverted_pos, *byte as char);
  });

  let _ = process_file("./data/data.txt", process_horiz, process_vert);

  let mut all_lines = Vec::new();

  for line in horiz_lm.borrow().lines.iter() {
    println!("Horiz line: {:?}", line);
    all_lines.push(line.clone());
  }

  for line in vert_lm.borrow().lines.iter() {
    println!("Vert line:  {:?}", line);
    all_lines.push(line.clone());
  }

  let mut rectangles = Vec::new();
  let mut leftover_lines = Vec::new();

  find_rectangles(&all_lines, &mut rectangles, &mut leftover_lines);

  for rect in rectangles.iter() {
    println!("Found rectangle: {:?}", rect);
  }

  for line in leftover_lines.iter() {
    println!("Leftover line: {:?}", line);
  }

  Ok(())
}
