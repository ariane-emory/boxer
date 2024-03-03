// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::process_file::*;
use boxer::simple_geo::find_rectangles;
use boxer::simple_geo::line_methods::*;
//use boxer::simple_geo::Word;
use std::io::{self};

////////////////////////////////////////////////////////////////////////////////////////////////////
static LINE_OFFSET: isize = 1;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  //loop {
  let filename = "./data/simple.box";
  let mut rectangles = Vec::new();
  let mut leftover_lines = Vec::new();
  let mut words = Vec::new();

  // all_lines scope:
  {
    let mut all_lines = Vec::new();

    // RefCell scope:
    {
      let (vert_linemaker, process_vert) =
        make_process_file_fun(b'|', b'-', false, false, |pos, byte| {
          println!(
            "Vert:    {:?}: '{}'",
            pos.flip().offset_by(LINE_OFFSET, 0),
            byte as char
          );
        });

      let (horiz_linemaker, process_horiz) =
        make_process_file_fun(b'-', b'|', true, true, |pos, byte| {
          println!(
            "Horiz:   {:?}: '{}'",
            pos.offset_by(LINE_OFFSET, 0),
            byte as char
          );
        });

      process_file(filename, process_horiz, process_vert)?;

      println!("");

      // we'll offset the line by one so that the line numbers are consistent
      // with emacs' line numbering.
      for line in horiz_linemaker.borrow().lines.iter() {
        let line = line.offset_by(LINE_OFFSET, 0);
        println!("Horiz line: {:?}", line);
        all_lines.push(line);
      }

      // we'll offset the line by one so that the line numbers are consistent
      // with emacs' line numbering. we'll also need to flip the row and
      // column on the vertical lines, since the LineMaker will have made
      // horizontal lines.
      for line in vert_linemaker.borrow().lines.iter() {
        let line = line.flip().offset_by(LINE_OFFSET, 0);
        println!("Vert line:  {:?}", line);
        all_lines.push(line);
      }

      // we'll offset the words too.
      for word in horiz_linemaker.borrow().words.iter() {
        words.push(word.offset_by(LINE_OFFSET, 0));
      }
    } // End of RefCell scope.

    find_rectangles(&all_lines, &mut rectangles, &mut leftover_lines, false);
  } // End of all_lines scope.

  println!("");

  for rect in rectangles.iter() {
    println!("Found rectangle: {:?}", rect);
  }

  for line in leftover_lines.iter() {
    println!("Leftover line:   {:?}", line);
  }

  for word in words.iter() {
    println!("Found word:      {:?}", word);
  }

  Ok(())
}
