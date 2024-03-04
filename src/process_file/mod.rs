use crate::line_makers::ConnectedLineMaker;
use crate::simple_geo::ConnectedLine;
use crate::simple_geo::Orientation;
use crate::simple_geo::Point;
use crate::simple_geo::Word;
use crate::simple_matrix::*;

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

/////////////////////////////////////////////////////////////////////////////////
pub fn make_process_matrix_bidirectionally_fun<'a>(
  orientation: Orientation,
  line_body_char: u8,
  wall_char: u8,
  collect_words: bool,
  allow_length_one: bool,
  line_postprocessor: impl Fn(ConnectedLine) -> ConnectedLine + 'a,
  word_postprocessor: impl Fn(Word) -> Word + 'a,
  custom_printer: impl Fn(Orientation, Point, u8) + 'a,
) -> (Rc<RefCell<ConnectedLineMaker<'a>>>, impl Fn(&Point, &u8) + 'a) {
  let linemaker = ConnectedLineMaker::new(
    orientation,
    line_body_char,
    wall_char,
    collect_words,
    allow_length_one,
    line_postprocessor,
    word_postprocessor,
  );
  let rc_linemaker = Rc::new(RefCell::new(linemaker));
  let rc_linemaker_twin = Rc::clone(&rc_linemaker);

  (rc_linemaker, move |pos: &Point, byte: &u8| {
    if pos.col == 0 {
      println!("");
    }

    if 0 != (*byte & 128) {
      panic!("Found non-ASCII byte {} at {:?}", byte, pos);
    }

    custom_printer(orientation, *pos, *byte);
    rc_linemaker_twin.borrow_mut().process(*pos, *byte);
  })
}

/////////////////////////////////////////////////////////////////////////////////
pub fn process_file(
  path: &str,
  process_horiz: impl Fn(&Point, &u8),
  process_vert: impl Fn(&Point, &u8),
) -> io::Result<(Vec<Vec<u8>>, Vec<ConnectedLine>, Vec<Rectangle>, Vec<Word>)> {
  let matrix: Vec<Vec<u8>> = read_file_to_byte_matrix(path)?;
  let max_len = matrix_max_row_len(&matrix);
  let uniform_matrix = normalize_matrix_width(&matrix, max_len, b' ');
  process_matrix_bidirectionally(&uniform_matrix, process_horiz, process_vert);

  let mut rectangles = Vec::new();
  let mut other_lines = Vec::new();
  let mut words = Vec::new();

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
      let (vert_linemaker, process_vert_fun) =
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

      let (horiz_linemaker, process_horiz_fun) =
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

  Ok((uniform_matrix, other_lines, rectangles, words))
}
