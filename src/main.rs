#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
mod simple_geo;
mod simple_matrix;
use simple_geo::*;
use simple_matrix::*;
use std::cmp::max;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{self, BufRead, BufReader};

////////////////////////////////////////////////////////////////////////////////////////////////////
trait FormatLines<T> {
  fn format_lines(&self) -> String;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl FormatLines<u8> for Vec<Vec<u8>> {
  fn format_lines(&self) -> String {
    let mut s: String = "[".to_string();

    if self.len() > 0 {
      s.push_str(" ");
      s.push_str(format!("\"{}\"", String::from_utf8_lossy(&self[0]).to_string()).as_str());

      for l in &self[1..] {
        s.push_str(format!(", \"{}\"", String::from_utf8_lossy(l).to_string()).as_str());
      }
      s.push_str(" ");
    }
    s.push_str("]");
    s
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
const NOISY: bool = false; // true;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn noisy_println(args: std::fmt::Arguments) {
  if NOISY {
    println!("{}", args);
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
macro_rules! noisy_println {
  ($($arg:tt)*) => {
    noisy_println(format_args!($($arg)*));
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_matrix<T>(byte_matrix: &Vec<Vec<T>>, process: Box<dyn Fn(&Point, T)>)
where
  T: Copy,
{
  let mut pos = Point::new(0, 0);

  loop {
    loop {
      let byte = byte_matrix[pos.line][pos.col];
      process(&pos, byte);

      pos.col += 1;

      if pos.col >= byte_matrix[pos.line].len() {
        break;
      }
    }

    pos.line += 1;
    pos.col = 0;

    noisy_println!("");

    if pos.line >= byte_matrix.len() {
      break;
    }
  }
}

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
fn read_file_to_byte_matrix(path: &str) -> io::Result<Vec<Vec<u8>>> {
  let file = File::open(path)?;
  let mut buf_reader = BufReader::new(file);
  let mut pos = Point::new(0, 0);
  let mut matrix: Vec<Vec<u8>> = Vec::new();

  // loop through the file and build the byte matrix:
  loop {
    let buffer: &[u8] = buf_reader.fill_buf()?;

    if buffer.is_empty() {
      break;
    }

    noisy_println!("-- ls:      {}", matrix.format_lines());
    noisy_println!("");

    let mut row: Vec<u8> = Vec::new();

    for &byte in buffer {
      if byte == b'\n' {
        noisy_println!("-- ls:      {}", matrix.format_lines());
        noisy_println!("-- c:       {:?}", pos.col);
        noisy_println!("-- l:       {:?}", pos.line);
        noisy_println!("");

        pos.col = 0;
        pos.line += 1;
        matrix.push(row);
        row = Vec::new();
      } else {
        row.push(byte);
        pos.col += 1;
      }
    }

    let len = buffer.len();
    buf_reader.consume(len);
  }

  pos.col = 0;
  pos.line = 0;

  noisy_println!("-- ls:  {}", matrix.format_lines());
  noisy_println!("");

  Ok(matrix)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_file_old(
  path: &str,
  process_horiz: Box<dyn Fn(&Point, u8)>,
  process_vert: Box<dyn Fn(&Point, u8)>,
) -> io::Result<()> {
  let file = File::open(path)?;
  let mut buf_reader = BufReader::new(file);
  let mut pos = Point::new(0, 0);
  let mut max_len = max_line_len(path)?;
  let mut columns: Vec<Vec<u8>> = Vec::new();

  noisy_println!("max_len:    {}", max_len);

  // loop through the file processing the bytes with process_horiz and building the columns:
  loop {
    let buffer: &[u8] = buf_reader.fill_buf()?;

    if buffer.is_empty() {
      break;
    }

    noisy_println!("-- ls:      {}", columns.format_lines());
    noisy_println!("");

    for &byte in buffer {
      if byte == b'\n' {
        while columns.len() < max_len {
          noisy_println!("Add an imaginary column with one byte and process it!");
          columns.push(vec![b' ']);
          process_horiz(&pos, b' ');
          pos.col += 1;
        }

        if pos.col < max_len {
          noisy_println!("Lengthen the imaginary column and process the new imaginary byte!");
          while pos.col < max_len {
            columns[pos.col].push(b' ');
            process_horiz(&pos, b' ');
            pos.col += 1;
          }
        }

        noisy_println!("-- ls:      {}", columns.format_lines());
        noisy_println!("-- c:       {:?}", pos.col);
        noisy_println!("-- l:       {:?}", pos.line);
        noisy_println!("");

        pos.col = 0;
        pos.line += 1;
      } else {
        process_horiz(&pos, byte);

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

  noisy_println!("-- ls:  {}", columns.format_lines());
  noisy_println!("");

  // loop through the columns, processing them with the process_vert function:
  process_matrix(&columns, process_vert);

  Ok(())
}

/////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_file(
  path: &str,
  process_horiz: Box<dyn Fn(&Point, u8)>,
  process_vert: Box<dyn Fn(&Point, u8)>,
) -> io::Result<()> {
  let max_len = max_line_len(path)?;
  let matrix: Vec<Vec<u8>> = read_file_to_byte_matrix(path)?;
  let uniform_matrix = make_matrix_uniform(&matrix, max_len, b' ');

  process_matrix(&uniform_matrix, process_horiz);

  let mut rotated_matrix = rotate_matrix(&uniform_matrix, Rotation::CounterClockwise);
  rotated_matrix.reverse();

  process_matrix(&rotated_matrix, process_vert);

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  let _ = process_file(
    "./data/data.txt",
    Box::new(|pos: &Point, byte: u8| {
      println!("Horiz {}:{}: '{}'", pos.col, pos.line, byte as char);
    }),
    Box::new(|pos: &Point, byte: u8| {
      println!("Vert  {}:{}: '{}'", pos.col, pos.line, byte as char);
    }),
  );

  println!("");

  let _ = process_file_old(
    "./data/data.txt",
    Box::new(|pos: &Point, byte: u8| {
      println!("Horiz {}:{}: '{}'", pos.col, pos.line, byte as char);
    }),
    Box::new(|pos: &Point, byte: u8| {
      println!("Vert  {}:{}: '{}'", pos.col, pos.line, byte as char);
    }),
  );

  Ok(())
}
