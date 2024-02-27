#![allow(unused_imports)]
#![allow(unused_variables)]
// #![allow(unused_mut)]
#![allow(dead_code)]
mod process_file;
mod simple_geo;
#[macro_use]
mod util;
mod simple_matrix;
use process_file::process_file;
use simple_geo::{Line, Point};
use std::cell::RefCell;
use std::io::{self};
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////
struct LineMaker {
  lines: Vec<Line>,
  line_begin: Option<Point>,
  line_body_char: u8,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl LineMaker {
  fn new(line_body_char: u8) -> LineMaker {
    LineMaker {
      lines: Vec::new(),
      line_begin: None,
      line_body_char,
    }
  }

  fn process(&mut self, pos: &Point, byte: &u8) {
    // feed a character to the LineMaker: this looks for ASCII art lines like '+----+'.-
    // When a '+' is observed and line_begin is None, the current position is recorded.
    // If line begin is set and the current character is the same as line_body_char, the
    // line is extended. If the current character is a '+', the line is closed and added
    // to the list of lines and line_begin is reset to None.
    // If some other character is observed in the middle (e.g., '+---a---+' the attempt
    // to create a line is abandoned (and line_begin becomes None).
    // A Line must contain at least one line_body character ('++' is not a line).

    match self.line_begin {
      None => {
        if *byte == b'+' {
          self.line_begin = Some(pos.clone());
        }
      }
      Some(begin) => {
        // in order to ensure that the line is at least one character long, we need to
        // check the distance between the current position and the line begin position:
        if *byte == b'+' && pos.distance(&begin) > 1 {
          let line = Line::from_points(&begin, &pos.clone()).unwrap();
          println!("new line: {:?}", line);
          self.lines.push(line);
          self.line_begin = None;
        } else if *byte != self.line_body_char {
          self.line_begin = None;
        }
      }
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  let lm = Rc::new(RefCell::new(LineMaker::new(b'-')));

  let process_horiz = Box::new(move |pos: &Point, byte: &u8| {
    if 0 != (*byte & 128) {
      panic!("Found non-ASCII byte {} at {:?}", byte, pos);
    }

    let mut lm = lm.borrow_mut();
    lm.process(pos, byte);
    println!("Horiz {:?}: '{}'", pos, *byte as char);
  });

  let lm = Rc::new(RefCell::new(LineMaker::new(b'|')));

  let process_vert = Box::new(move |pos: &Point, byte: &u8| {
    let mut lm = lm.borrow_mut();
    let inverted_pos = Point::new(pos.line, pos.col);
    lm.process(&inverted_pos, byte);
    println!("Vert  {:?}: '{}'", inverted_pos, *byte as char);
  });

  let _ = process_file("./data/data.txt", process_horiz, process_vert);

  Ok(())
}
