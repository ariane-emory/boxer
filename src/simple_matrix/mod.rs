use crate::simple_geo::Point;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq)]
pub enum Rotation {
  Clockwise,
  CounterClockwise,
}

////////////////////////////////////////////////////////////////////////////////
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
      }
      else {
        rotated_matrix[num_cols - 1 - col][row] = matrix[row][col];
      }
    }
  }

  rotated_matrix
}

////////////////////////////////////////////////////////////////////////////////
pub fn normalize_matrix_width<T>(
  byte_matrix: &Vec<Vec<T>>,
  len: usize,
  val: T,
) -> Vec<Vec<T>>
where
  T: Copy,
{
  let mut new_matrix = Vec::new();

  for row in byte_matrix {
    let row_len = row.len();

    if row_len < len {
      let mut new_row = row.to_vec();
      new_row.resize(len, val);
      new_matrix.push(new_row);
    }
    else {
      let new_row = row[0..len.min(row_len)].to_vec();
      new_matrix.push(new_row);
    }
  }

  new_matrix
}

////////////////////////////////////////////////////////////////////////////////
pub fn matrix_max_row_len<T>(byte_matrix: &Vec<Vec<T>>) -> usize {
  let mut max = 0;

  for row in byte_matrix {
    let len = row.len();

    if len > max {
      max = len;
    }
  }

  max
}

/////////////////////////////////////////////////////////////////////////////////
// This fn assumes matrix is already uniform:
pub fn process_matrix_bidirectionally(
  matrix: &Vec<Vec<u8>>,
  process_horiz: impl Fn(&Point, &u8),
  process_vert: impl Fn(&Point, &u8),
) {
  let mut rotated_matrix = rotate_matrix(&matrix, Rotation::CounterClockwise);
  rotated_matrix.reverse();
  rotated_matrix.each(process_vert);
  println!("\n================================================================================");
  matrix.each(process_horiz);
}

/////////////////////////////////////////////////////////////////////////////////
pub fn read_file_to_byte_matrix(path: &str) -> io::Result<Vec<Vec<u8>>> {
  let file = File::open(path)?;
  let buf_reader = BufReader::new(file);
  let mut matrix = Vec::new();

  for line in buf_reader.lines() {
    let line = line?;
    matrix.push(line.into_bytes());
  }

  Ok(matrix)
}

////////////////////////////////////////////////////////////////////////////////
pub trait FormatRows<T> {
  #[allow(dead_code)]
  fn format_rows(&self) -> String;
}

////////////////////////////////////////////////////////////////////////////////
impl FormatRows<u8> for Vec<Vec<u8>> {
  fn format_rows(&self) -> String {
    let mut s: String = "[".to_string();

    if self.len() > 0 {
      s.push_str(" ");
      s.push_str(
        format!("\"{}\"", String::from_utf8_lossy(&self[0]).to_string())
          .as_str(),
      );

      for l in &self[1..] {
        s.push_str(
          format!(", \"{}\"", String::from_utf8_lossy(l).to_string()).as_str(),
        );
      }
      s.push_str(" ");
    }
    s.push_str("]");
    s
  }
}

////////////////////////////////////////////////////////////////////////////////
pub trait MatrixEachable<T> {
  fn each(&self, process: impl Fn(&Point, &T));
}

////////////////////////////////////////////////////////////////////////////////
impl<T> MatrixEachable<T> for Vec<Vec<T>> {
  fn each(&self, process: impl Fn(&Point, &T)) {
    let mut pos = Point::new(0, 0);

    for line in 0..self.len() {
      pos.col = 0;

      for col in 0..self[line].len() {
        pos.line = line;
        pos.col = col;

        let elem = &self[line][col];

        process(&pos, elem);
      }
    }
  }
}
