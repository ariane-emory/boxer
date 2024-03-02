use crate::simple_geo::Line;
use crate::simple_geo::Point;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
pub struct LineMaker {
  pub lines: Vec<Line>,
  line_begin: Option<Point>,
  line_body_char: u8,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl LineMaker {
  #[allow(dead_code)]
  pub fn new(line_body_char: u8) -> LineMaker {
    LineMaker {
      lines: Vec::new(),
      line_begin: None,
      line_body_char,
    }
  }

  #[allow(dead_code)]
  pub fn process(
    &mut self,
    pos: &Point,
    byte: u8,
  ) {
    // Feed a character to the LineMaker: this looks for ASCII art lines like '+----+'.-
    // When a '+' is observed and line_begin is None, the current position is recorded.
    // If line begin is set and the current character is the same as line_body_char, the
    // line is extended. If the current character is a '+', the line is closed and added
    // to the list of lines and line_begin is reset to None.
    // If some other character is observed in the middle (e.g., '+---a---+' the attempt
    // to create a line is abandoned (and line_begin becomes None).
    // A Line must contain at least one line_body character ('++' is not a line).
    if pos.col == 0 {
      println!("         new line!");
      self.line_begin = None;
    }

    if let Some(begin) = self.line_begin {
      // in order to ensure that the line is at least one character long, we need to
      // check the distance between the current position and the line begin position:
      if byte == b'+' && pos.distance(&begin) > 1 {
        let line = Line::new(begin, *pos).unwrap();
        println!("         CREATE LINE: {:?}", line);
        self.lines.push(line);
        self.line_begin = None;
      } else if byte != self.line_body_char {
        println!("         broke line, distance = {}!", pos.distance(&begin));
        self.line_begin = None;
      }
    } else if byte == b'+' {
      self.line_begin = Some(*pos);
    }
  }
}
