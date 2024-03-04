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
  let mut rectangles = Vec::new();
  let mut other_lines = Vec::new();
  let mut words = Vec::new();
  let mut _matrix = Vec::new();

  // all_lines scope:
  {
    let mut all_lines: Vec<ConnectedLine> = Vec::new();
    let offset_pos = |pos: Point| pos.flip().offset_by(LINE_OFFSET, 0);
    let offset_line = |cl: ConnectedLine| cl.offset_by(LINE_OFFSET, 0);
    let offset_word = |wrd: Word| wrd.offset_by(LINE_OFFSET, 0);
    let flip_and_offset_pos = |pos: Point| offset_pos(pos).flip();
    let flip_and_offset_line = |cl: ConnectedLine| offset_line(cl.flip());
    let flip_and_offset_word = |wrd: Word| offset_word(wrd.flip());
    let log_labeled_byte = |ori: Orientation, pos: Point, byte: u8| {
      println!("{:12} {:?}: '{}'", format!("{:?}:", ori), pos, byte as char)
    };
    let log_byte_with_orientation_and_offset_pos =
      |ori, pos, byte| log_labeled_byte(ori, offset_pos(pos), byte);
    let log_byte_with_orientation_and_flipped_and_offset_pos =
      |ori, pos, byte| log_labeled_byte(ori, flip_and_offset_pos(pos), byte);

    // RefCell scope:
    {
      let (vert_linemaker, process_vert) =
        make_process_matrix_bidirectionally_fun(
          Vertical,
          b'|',
          b'-',
          false,
          false,
          flip_and_offset_line,
          flip_and_offset_word,
          log_byte_with_orientation_and_flipped_and_offset_pos,
        );

      let (horiz_linemaker, process_horiz) =
        make_process_matrix_bidirectionally_fun(
          Horizontal,
          b'-',
          b'|',
          true,
          true,
          offset_line,
          offset_word,
          log_byte_with_orientation_and_offset_pos,
        );

      _matrix = process_file(filename, process_horiz, process_vert)?;

      println!("");

      words.extend(horiz_linemaker.borrow().words.iter().cloned());
      all_lines.extend(horiz_linemaker.borrow().lines.iter());
      all_lines.extend(vert_linemaker.borrow().lines.iter());
    } // End of RefCell scope.

    other_lines.extend(all_lines.iter().filter(|cl| !cl.corner_connected()));
    all_lines.retain(ConnectedLine::corner_connected);

    find_rectangles(&all_lines, &mut rectangles, &mut other_lines, false);
  } // End of all_lines scope.

  println!("");

  other_lines
    .iter()
    .for_each(|line| println!("Other line:      {:?}", line));
  words
    .iter()
    .for_each(|word| println!("Found word:      {:?}", word));
  rectangles
    .iter()
    .for_each(|rect| println!("Found rectangle: {:?}", rect));

  Ok(())
}
