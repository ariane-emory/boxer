use crate::simple_geo::Point;
use crate::simple_matrix::*;
use crate::util::max_line_len;

use std::io::{self};

/////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn process_file(
  path: &str,
  process_horiz: Box<dyn Fn(&Point, &u8)>,
  process_vert: Box<dyn Fn(&Point, &u8)>,
) -> io::Result<()> {
  let max_len = max_line_len(path)?;
  let matrix: Vec<Vec<u8>> = read_file_to_byte_matrix(path)?;
  let uniform_matrix = normalize_matrix_width(&matrix, max_len, b' ');

  uniform_matrix.each(process_horiz);

  let mut rotated_matrix = rotate_matrix(&uniform_matrix, Rotation::CounterClockwise);
  rotated_matrix.reverse();

  rotated_matrix.each(process_vert);

  Ok(())
}
