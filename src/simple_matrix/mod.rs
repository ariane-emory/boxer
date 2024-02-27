use crate::simple_geo::Point;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

////////////////////////////////////////////////////////////////////////////////////////////////////
const NOISY: bool = false; // true;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Temporarily public, move this somewhere else!
pub fn noisy_println(args: std::fmt::Arguments) {
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
#[derive(Debug, PartialEq)]
pub enum Rotation {
  Clockwise,
  CounterClockwise,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub use Rotation::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn rotate_matrix<T>(matrix: &[Vec<T>], rot: Rotation) -> Vec<Vec<T>>
where
  T: Copy,
{
  let num_rows = matrix.len();

  if num_rows == 0 {
    return vec![];
  }

  let num_cols = matrix[0].len();

  let mut rotated_matrix = vec![vec![matrix[0][0]; num_rows]; num_cols];

  for row in 0..num_rows {
    for col in 0..num_cols {
      if rot == Rotation::Clockwise {
        rotated_matrix[col][num_rows - 1 - row] = matrix[row][col];
      } else {
        rotated_matrix[num_cols - 1 - col][row] = matrix[row][col];
      }
    }
  }

  rotated_matrix
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn make_matrix_uniform<T>(byte_matrix: &Vec<Vec<T>>, len: usize, val: T) -> Vec<Vec<T>>
where
  T: Copy,
{
  let mut new_matrix = Vec::new();

  for row in byte_matrix {
    let row_len = row.len();
    if row_len < len {
      let mut new_row = row.clone();
      new_row.resize(len, val);
      new_matrix.push(new_row);
    } else {
      let new_row = row[0..len.min(row_len)].to_vec();
      new_matrix.push(new_row);
    }
  }

  new_matrix
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait FormatRows<T> {
  fn format_lines(&self) -> String;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl FormatRows<u8> for Vec<Vec<u8>> {
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

/////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn read_file_to_byte_matrix(path: &str) -> io::Result<Vec<Vec<u8>>> {
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
pub fn process_matrix<T>(byte_matrix: &Vec<Vec<T>>, process: Box<dyn Fn(&Point, T)>)
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
