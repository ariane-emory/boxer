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
