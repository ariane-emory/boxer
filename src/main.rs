#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
mod simple_geo;
use simple_geo::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

////////////////////////////////////////////////////////////////////////////////////////////////////
trait FormatLines {
  fn format_lines(&self) -> String;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl FormatLines for Vec<Vec<u8>> {
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
const NOISY: bool = true;

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
fn process_file<R: BufRead>(
  buf_reader: R,
  process_horiz: Box<dyn Fn(&Point, u8)>,
  process_vert: Box<dyn Fn(&Point, u8)>,
) -> io::Result<()> {
  let file = File::open("./data/data.txt")?;
  let mut buf_reader = BufReader::new(file);
  let mut lines: Vec<Vec<u8>> = Vec::new();
  let mut pos = Point::new(0, 0);

  loop {
    let buffer: &[u8] = buf_reader.fill_buf()?;

    if buffer.is_empty() {
      break;
    }

    noisy_println!("-- ls:  {}", lines.format_lines());
    noisy_println!("");

    for &byte in buffer {
      if byte == b'\n' {
        pos.col = 0;
        pos.line += 1;

        noisy_println!("-- c:   {:?}", pos.col);
        noisy_println!("-- l:   {:?}", pos.line);
        noisy_println!("-- ls:  {}", lines.format_lines());
        noisy_println!("");
      } else {
        process_horiz(&pos, byte);

        if lines.len() > pos.col {
          lines[pos.col].push(byte);
        } else {
          lines.push(vec![byte]);
        }

        pos.col += 1;
      }
    }
    let len = buffer.len();
    buf_reader.consume(len);
  }

  pos.col = 0;
  pos.line = 0;

  noisy_println!("-- ls:  {}", lines.format_lines());
  noisy_println!("");

  loop {
    loop {
      let byte = lines[pos.line][pos.col];
      process_vert(&pos, byte);

      pos.col += 1;

      if pos.col >= lines[pos.line].len() {
        break;
      }
    }

    pos.line += 1;
    pos.col = 0;

    noisy_println!("");

    if pos.line >= lines.len() {
      break;
    }
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  process_file(
    BufReader::new(File::open("./data/data.txt")?),
    Box::new(|pos: &Point, byte: u8| {
      println!("Horiz {}:{}: '{}'", pos.col, pos.line, byte as char);
    }),
    Box::new(|pos: &Point, byte: u8| {
      println!("Vert  {}:{}: '{}'", pos.col, pos.line, byte as char);
    }),
  )
}
