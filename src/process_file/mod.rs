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
) -> io::Result<Vec<Vec<u8>>> {
  let matrix: Vec<Vec<u8>> = read_file_to_byte_matrix(path)?;
  let max_len = matrix_max_row_len(&matrix);
  let uniform_matrix = normalize_matrix_width(&matrix, max_len, b' ');
  process_matrix_bidirectionally(&uniform_matrix, process_horiz, process_vert);
  Ok(uniform_matrix)
}
