use crate::line_makers::ConnectedLineMaker;
use crate::simple_geo::Point;
use crate::simple_matrix::*;
use crate::util::max_line_len;
use std::cell::RefCell;
use std::io::{self};
use std::rc::Rc;

/////////////////////////////////////////////////////////////////////////////////
pub fn process_file(
  path: &str,
  process_horiz: impl Fn(&Point, &u8),
  process_vert: impl Fn(&Point, &u8),
) -> io::Result<()> {
  let max_len = max_line_len(path)?;
  let matrix: Vec<Vec<u8>> = read_file_to_byte_matrix(path)?;
  let uniform_matrix = normalize_matrix_width(&matrix, max_len, b' ');
  let mut rotated_matrix =
    rotate_matrix(&uniform_matrix, Rotation::CounterClockwise);
  rotated_matrix.reverse();

  rotated_matrix.each(process_vert);
  uniform_matrix.each(process_horiz);

  Ok(())
}

/////////////////////////////////////////////////////////////////////////////////
pub fn make_process_file_fun<'a>(
  line_body_char: u8,
  wall_char: u8,
  custom_printer: impl Fn(Point, u8) + 'a,
) -> (Rc<RefCell<ConnectedLineMaker>>, impl Fn(&Point, &u8) + 'a) {
  let lm = ConnectedLineMaker::new(line_body_char, wall_char);
  let rc_lm = Rc::new(RefCell::new(lm));
  let rc_lm_twin = Rc::clone(&rc_lm);

  (rc_lm, move |pos: &Point, byte: &u8| {
    if pos.col == 0 {
      println!("");
    }

    if 0 != (*byte & 128) {
      panic!("Found non-ASCII byte {} at {:?}", byte, pos);
    }

    custom_printer(*pos, *byte);
    rc_lm_twin.borrow_mut().process(*pos, *byte);
  })
}
