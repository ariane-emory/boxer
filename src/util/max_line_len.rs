use crate::simple_geo::Point;
use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

////////////////////////////////////////////////////////////////////////////////
pub fn max_line_len_file(path: &str) -> io::Result<usize> {
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
      }
      else {
        pos.col += 1;
      }
    }

    let len = buffer.len();
    buf_reader.consume(len);
  }

  Ok(max_len)
}
