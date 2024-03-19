////////////////////////////////////////////////////////////////////////////////
pub fn add_null_sentinels_to_normalized_matrix(
  mut matrix: Vec<Vec<u8>>,
) -> Vec<Vec<u8>> {
  let normalized_matrix_width = matrix[0].len();
  let terminator = b'\0';

  for row in matrix.iter_mut() {
    row.push(terminator);
  }

  matrix.push(vec![terminator; normalized_matrix_width + 1]);
  matrix
}
