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
#[derive(Debug, PartialEq)]
pub enum Rotation {
  Clockwise,
  CounterClockwise,
}
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
