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
  println!("jkkkkkkkk");
  let max_len = max_line_len(path)?;
  println!("NNNNNk");
  let matrix: Vec<Vec<u8>> = read_file_to_byte_matrix(path)?;
  println!("jkkkkkkkk");
  let uniform_matrix = normalize_matrix_width(&matrix, max_len, b' ');
  println!("jkkkkkkkk");
  uniform_matrix.each(process_horiz);
  println!("jkkkkkkkk");

  let mut rotated_matrix = rotate_matrix(&uniform_matrix, Rotation::CounterClockwise);
  println!("jkkkkkkkk");
  rotated_matrix.reverse();
  println!("jkkkkkkkk");

  rotated_matrix.each(process_vert);
  println!("jkkkkkkkk");

  Ok(())
}
