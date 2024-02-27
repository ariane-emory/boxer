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
use std::cmp::max;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{self, BufRead, BufReader};
type Point = simple_geo::Point;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn max_line_len(path: &str) -> io::Result<usize> {
  let file = File::open(path)?;
  let mut buf_reader = BufReader::new(file);
  let mut pos = Point::new(0, 0);
  let mut max_len = 0;

  loop {
    let buffer: &[u8] = buf_reader.fill_buf()?;

    if buffer.is_empty() {
      break;
    }

    for &byte in buffer {
      if byte == b'\n' {
        max_len = max(max_len, pos.col);
        pos.col = 0;
        pos.line += 1;
      } else {
        pos.col += 1;
      }
    }

    let len = buffer.len();
    buf_reader.consume(len);
  }

  Ok(max_len)
}

/////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_file(
  path: &str,
  process_horiz: Box<dyn Fn(&Point, &u8)>,
  process_vert: Box<dyn Fn(&Point, &u8)>,
) -> io::Result<()> {
  let max_len = max_line_len(path)?;
  let matrix: Vec<Vec<u8>> = read_file_to_byte_matrix(path)?;
  let uniform_matrix = make_matrix_uniform(&matrix, max_len, b' ');

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
      println!("Vert  {:?}: '{}'", pos, *byte as char);
    }),
  );

  Ok(())
}
