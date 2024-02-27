#![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
mod util;
use util::*;
mod simple_geo;
#[macro_use]
mod simple_matrix;
use simple_geo::*;
use simple_matrix::*;
use std::io::{self};
type Point = simple_geo::Point;

/////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_file(
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

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  let _ = process_file(
    "./data/data.txt",
    Box::new(|pos: &Point, byte: &u8| {
      println!("Horiz {:?}: '{}'", pos, *byte as char);
    }),
    Box::new(|pos: &Point, byte: &u8| {
      // Print an inverted posittion to reflext the character's original posiion in the file:
      let inverted_pos = Point::new(pos.line, pos.col);
      println!("Vert  {:?}: '{}'", inverted_pos, *byte as char);
    }),
  );

  Ok(())
}
