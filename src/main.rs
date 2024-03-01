#![allow(unreachable_code)]
//#![allow(unused_imports)]
//#![allow(unused_variables)]
//#![allow(unused_mut)]
//#![allow(dead_code)]

mod block;
mod line_makers; //::connected_line_maker;
mod process_file;
mod simple_geo;
#[macro_use]
mod util;
mod simple_matrix;

//use block::*;
use process_file::*;
use simple_geo::find_rectangles;
use simple_geo::line_methods::*;
use std::io::{self};

////////////////////////////////////////////////////////////////////////////////////////////////////
static LINE_OFFSET: isize = 1;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  let filename = "./data/tangle.box";
  let mut rectangles = Vec::new();
  let mut leftover_lines = Vec::new();

  // all_lines scope:
  {
    let mut all_lines = Vec::new();

    // RefCell scope:
    {
      let (vert_linemaker, process_vert) = make_process_file_fun(b'|', |pos, byte| {
        println!(
          "Vert:    {:?}: '{}'",
          pos.flip().offset_by(LINE_OFFSET, 0),
          byte as char
        );
      });
      let (horiz_linemaker, process_horiz) = make_process_file_fun(b'-', |pos, byte| {
        println!(
          "Horiz:   {:?}: '{}'",
          pos.offset_by(LINE_OFFSET, 0),
          byte as char
        );
      });

      process_file(filename, process_horiz, process_vert)?;

      println!("");

      // we'll offset the line by one so that the line numbers are consistent with emacs'
      // line numbering.
      for line in horiz_linemaker.borrow().lines.iter() {
        let line = line.offset_by(LINE_OFFSET, 0);
        println!("Horiz line: {:?}", line);
        all_lines.push(line);
      }

      // we'll offset the line by one so that the line numbers are consistent with emacs'
      // line numbering. we'll also need to flip the row and column on the vertical lines,
      // since the LineMaker will have made horizontal lines.
      for line in vert_linemaker.borrow().lines.iter() {
        let line = line.flip().offset_by(LINE_OFFSET, 0);
        println!("Vert line:  {:?}", line);
        all_lines.push(line);
      }
    } // End of RefCell scope.

    find_rectangles(&all_lines, &mut rectangles, &mut leftover_lines);
  } // End of all_lines scope.

  for rect in rectangles.iter() {
    println!("Found rectangle: {:?}", rect);
  }

  for line in leftover_lines.iter() {
    println!("Leftover line: {:?}", line);
  }

  // let one = block::Value::new(1);
  // let five = block::Value::new(5);
  // let mut adder = block::MathAdd::new(&one.output, &five.output);
  // println!("Adder: {}", adder.output.read());
  // adder.step();
  // println!("Adder: {}", adder.output.read());

  // let mut flip = block::Value::new(false);
  // let ten = block::Value::new(10);
  // let mut ctr = block::RiseCounter::new(&flip.output, &ten.output);
  // println!("Ctr: {}", ctr.count.read());
  // ctr.step();
  // println!("Ctr: {}", ctr.count.read());
  // flip.output.set(true);
  // ctr.step();
  // println!("Ctr: {}", ctr.count.read());

  Ok(())
}
