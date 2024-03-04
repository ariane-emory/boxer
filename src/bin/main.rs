// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::process_file::*;
use boxer::simple_geo::find_rectangles;
use boxer::simple_geo::ConnectedLine;
use boxer::simple_geo::ConnectionType::*;
use boxer::simple_geo::Offsetable;
use boxer::simple_geo::Orientation;
use boxer::simple_geo::Orientation::*;
use boxer::simple_geo::Point;
use boxer::simple_geo::Word;
//use boxer::simple_geo::Word;
//use boxer::simple_geo::LineMethods;
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
  let mut _matrix = Vec::new();

  // all_lines scope:
  {
    let offset_line = |cl: ConnectedLine| cl.offset_by(LINE_OFFSET, 0);
    let flip_and_offset_line = |cl: ConnectedLine| offset_line(cl.flip());
    let offset_word = |wrd: Word| wrd.offset_by(LINE_OFFSET, 0);
    let flip_and_offset_word = |wrd: Word| offset_word(wrd.flip());
    let offset_pos = |pos: Point| pos.flip().offset_by(LINE_OFFSET, 0);
    let flip_and_offset_pos = |pos: Point| offset_pos(pos).flip();
    let log_labeled_byte = |ori: Orientation, pos: Point, byte: u8| {
      println!("{:12} {:?}: '{}'", format!("{:?}:", ori), pos, byte as char)
    };
    let log_byte_with_orientation_and_flipped_and_offset_pos =
      |ori, pos, byte| log_labeled_byte(ori, flip_and_offset_pos(pos), byte);
    let log_byte_with_orientation_and_offset_pos =
      |ori, pos, byte| log_labeled_byte(ori, offset_pos(pos), byte);
    let mut all_lines = Vec::new();

    // RefCell scope:
    {
      let (vert_linemaker, process_vert) = make_process_file_fun(
        Vertical,
        b'|',
        b'-',
        false,
        false,
        flip_and_offset_line,
        flip_and_offset_word,
        log_byte_with_orientation_and_flipped_and_offset_pos,
      );

      let (horiz_linemaker, process_horiz) = make_process_file_fun(
        Horizontal,
        b'-',
        b'|',
        true,
        true,
        offset_line,
        offset_word,
        log_byte_with_orientation_and_offset_pos,
      );

      _matrix = process_file(filename, process_horiz, process_vert);

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

    let corner_connected = |cl: &ConnectedLine| {
      cl.start_connects_to == Corner && cl.end_connects_to == Corner
    };

    leftover_lines
      .extend(all_lines.iter().filter(|line| !corner_connected(line)));
    all_lines.retain(corner_connected);

    find_rectangles(&all_lines, &mut rectangles, &mut leftover_lines, false);
  } // End of all_lines scope.

  println!("");

  for line in leftover_lines.iter() {
    println!("Leftover line:   {:?}", line);
  }

  for word in words.iter() {
    println!("Found word:      {:?}", word);
  }

  for rect in rectangles.iter() {
    println!("Found rectangle: {:?}", rect);
  }

  Ok(())
}
