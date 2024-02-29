#![allow(unreachable_code)]
//#![allow(unused_imports)]
//#![allow(unused_variables)]
//#![allow(unused_mut)]
//#![allow(dead_code)]

mod line_makers; //::anchored_line_maker;
mod process_file;
mod simple_geo;
#[macro_use]
mod util;
mod simple_matrix;

use line_makers::AnchoredLineMaker;

use process_file::process_file;
use simple_geo::find_rectangles;
use simple_geo::AnchoredLine;
use simple_geo::Point;
use std::cell::RefCell;
use std::io::{self};
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  let filename = "./data/single.box";
  let mut rectangles = Vec::new();
  let mut leftover_lines = Vec::new();

  // all_lines scope:
  {
    let _line_offset = 1;
    let mut all_lines = Vec::new();

    // Closure/RefCell scope:
    {
      let vert_linemaker = Rc::new(RefCell::new(AnchoredLineMaker::new(b'|')));
      let vert_linemaker_twin = Rc::clone(&vert_linemaker);
      let process_vert = Box::new(move |pos: &Point, byte: &u8| {
        if pos.col == 0 {
          println!("");
        }

        if 0 != (*byte & 128) {
          panic!("Found non-ASCII byte {} at {:?}", byte, pos);
        }

        let mut lm = vert_linemaker_twin.borrow_mut();
        lm.process(pos, byte);

        println!("Vert  {:?}: '{}'", pos, *byte as char);
      });

      let horiz_linemaker = Rc::new(RefCell::new(AnchoredLineMaker::new(b'-')));
      let horiz_linemaker_twin = Rc::clone(&horiz_linemaker);
      let process_horiz = Box::new(move |pos: &Point, byte: &u8| {
        if pos.col == 0 {
          println!("");
        }

        // Don't bother checking if byte is in the ASCII range since it was already checked during
        // the vertical pass.

        let mut lm = horiz_linemaker_twin.borrow_mut();
        lm.process(pos, byte);

        println!("Horiz {:?}: '{}'", pos, *byte as char);
      });

      process_file(filename, process_horiz, process_vert)?;

      println!("");

      for line in horiz_linemaker.borrow().lines.iter() {
        println!("Horiz line: {:?}", line);
        all_lines.push(*line);
      }

      for line in vert_linemaker.borrow().lines.iter() {
        // we need to flip the row and column on the vertical lines, since the LineMaker will have
        // made horizontal lines.

        let line = AnchoredLine::new(
          line.start.col,
          line.start.line,
          line.end.col,
          line.end.line,
          line.anchoring,
        )
        .unwrap();

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
