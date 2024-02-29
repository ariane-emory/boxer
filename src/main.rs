#![allow(unreachable_code)]
//#![allow(unused_imports)]
//#![allow(unused_variables)]
//#![allow(unused_mut)]
//#![allow(dead_code)]

mod line_makers; //::connected_line_maker;
mod process_file;
mod simple_geo;
#[macro_use]
mod util;
mod simple_matrix;

use line_makers::ConnectedLineMaker;
use process_file::process_file;
use simple_geo::find_rectangles;
use simple_geo::line_methods::*;
use simple_geo::Point;
use std::cell::RefCell;
use std::io::{self};
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn make_process_fun(
  lm: ConnectedLineMaker,
) -> (Rc<RefCell<ConnectedLineMaker>>, Box<dyn Fn(&Point, &u8)>) {
  let rclm = Rc::new(RefCell::new(lm));
  let twin = Rc::clone(&rclm);

  (
    rclm,
    Box::new(move |pos: &Point, byte: &u8| {
      if pos.col == 0 {
        println!("");
      }

      if 0 != (*byte & 128) {
        panic!("Found non-ASCII byte {} at {:?}", byte, pos);
      }

      let mut lm = twin.borrow_mut();
      lm.process(pos, byte);

      println!("Horiz {:?}: '{}'", pos, *byte as char);
    }),
  )
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  let filename = "./data/single.box";
  let mut rectangles = Vec::new();
  let mut leftover_lines = Vec::new();

  // all_lines scope:
  {
    let mut all_lines = Vec::new();

    // Closure/RefCell scope:
    {
      let (vert_linemaker, process_vert) = make_process_fun(ConnectedLineMaker::new(b'|'));
      let (horiz_linemaker, process_horiz) = make_process_fun(ConnectedLineMaker::new(b'-'));

      process_file(filename, process_horiz, process_vert)?;

      println!("");

      let line_offset = 1;

      // we'll offset the line by one so that the line numbers are consistent with emacs'
      // line numbering.
      for line in horiz_linemaker.borrow().lines.iter() {
        let line = line.offset_by(line_offset, 0);
        println!("Horiz line: {:?}", line);
        all_lines.push(line);
      }

      // we'll offset the line by one so that the line numbers are consistent with emacs'
      // line numbering. we'll also need to flip the row and column on the vertical lines,
      // since the LineMaker will have made horizontal lines.
      for line in vert_linemaker.borrow().lines.iter() {
        let line = line.flip().offset_by(line_offset, 0);
        println!("Vert line:  {:?}", line);
        all_lines.push(line);
      }
    } // End of closure/RefCell scope.

    find_rectangles(&all_lines, &mut rectangles, &mut leftover_lines);
  } // End of all_lines scope.

  for rect in rectangles.iter() {
    println!("Found rectangle: {:?}", rect);
  }

  for line in leftover_lines.iter() {
    println!("Leftover line: {:?}", line);
  }

  Ok(())
}
