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

////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_file_old(
  path: &str,
  process_horiz: Box<dyn Fn(&Point, &u8)>,
  process_vert: Box<dyn Fn(&Point, &u8)>,
) -> io::Result<()> {
  let file = File::open(path)?;
  let max_len = max_line_len(path)?;
  let mut buf_reader = BufReader::new(file);
  let mut pos = Point::new(0, 0);
  let mut columns: Vec<Vec<u8>> = Vec::new();

  noisy_println!("max_len:    {}", max_len);

  // loop through the file processing the bytes with process_horiz and building the columns:
  loop {
    let buffer: &[u8] = buf_reader.fill_buf()?;

    if buffer.is_empty() {
      break;
    }

    noisy_println!("-- ls:      {}", columns.format_rows());
    noisy_println!("");

    for &byte in buffer {
      if byte == b'\n' {
        while columns.len() < max_len {
          noisy_println!("Add an imaginary column with one byte and process it!");
          columns.push(vec![b' ']);
          process_horiz(&pos, &b' ');
          pos.col += 1;
        }

        if pos.col < max_len {
          noisy_println!("Lengthen the imaginary column and process the new imaginary byte!");
          while pos.col < max_len {
            columns[pos.col].push(b' ');
            process_horiz(&pos, &b' ');
            pos.col += 1;
          }
        }

        noisy_println!("-- ls:      {}", columns.format_rows());
        noisy_println!("-- c:       {:?}", pos.col);
        noisy_println!("-- l:       {:?}", pos.line);
        noisy_println!("");

        pos.col = 0;
        pos.line += 1;
      } else {
        process_horiz(&pos, &byte);

        if columns.len() > pos.col {
          columns[pos.col].push(byte);
        } else {
          columns.push(vec![byte]);
        }

        pos.col += 1;
      }
    }
    let len = buffer.len();
    buf_reader.consume(len);
  }

  pos.col = 0;
  pos.line = 0;

  noisy_println!("-- ls:  {}", columns.format_rows());
  noisy_println!("");

  // loop through the columns, processing them with the process_vert function:
  columns.each(process_vert);

  Ok(())
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
      println!("Horiz {}:{}: '{}'", pos.col, pos.line, *byte as char);
    }),
    Box::new(|pos: &Point, byte: &u8| {
      println!("Vert  {}:{}: '{}'", pos.col, pos.line, *byte as char);
    }),
  );

  if false {
    println!("");

    let _ = process_file_old(
      "./data/data.txt",
      Box::new(|pos: &Point, byte: &u8| {
        println!("Horiz {}:{}: '{}'", pos.col, pos.line, *byte as char);
      }),
      Box::new(|pos: &Point, byte: &u8| {
        println!("Vert  {}:{}: '{}'", pos.col, pos.line, *byte as char);
      }),
    );
  }

  Ok(())
}
