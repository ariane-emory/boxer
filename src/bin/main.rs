// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::process_file::*;
use boxer::simple_geo::find_rectangles;
use boxer::simple_geo::Orientation::*;
//use boxer::simple_geo::Word;
use boxer::simple_geo::LineMethods;
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
      let (vert_linemaker, process_vert) = make_process_file_fun(
        Vertical,
        b'|',
        b'-',
        false,
        false,
        // Offset the line nunbers to match emacs' numbering:
        Box::new(|line| {
          let l = line.flip().offset_by(LINE_OFFSET, 0);
          println!("Changed {:?} into {:?}!", line, l);
          l
        }),
        Box::new(|word| word.offset_by(LINE_OFFSET, 0)),
        |pos, byte| {
          println!(
            "Vert:    {:?}: '{}'",
            pos.flip().offset_by(LINE_OFFSET, 0),
            byte as char
          );
        },
      );

      let (horiz_linemaker, process_horiz) = make_process_file_fun(
        Horizontal,
        b'-',
        b'|',
        true,
        true,
        // Offset the line nunbers to match emacs' numbering:
        Box::new(|line| {
          let l = line.offset_by(LINE_OFFSET, 0);
          println!("Changed {:?} into {:?}", line, l);
          l
        }),
        Box::new(|word| word.offset_by(LINE_OFFSET, 0)),
        |pos, byte| {
          println!(
            "Horiz:   {:?}: '{}'",
            pos.offset_by(LINE_OFFSET, 0),
            byte as char
          );
        },
      );

      process_file(filename, process_horiz, process_vert)?;

      println!("");

      for line in horiz_linemaker.borrow().lines.iter() {
        println!("Horiz line: {:?}", line);
        all_lines.push(*line);
      }

      for line in vert_linemaker.borrow().lines.iter() {
        println!("Vert line:  {:?}", line);
        all_lines.push(*line);
      }

      for word in horiz_linemaker.borrow().words.iter() {
        words.push(word.clone());
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
