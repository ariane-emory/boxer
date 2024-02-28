use crate::simple_geo::AnchoredLine;
use crate::simple_geo::Anchoring::Both;
use crate::simple_geo::Point;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct AnchoredLineMaker {
  pub lines: Vec<AnchoredLine>,
  line_begin: Option<Point>,
  line_body_char: u8,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl AnchoredLineMaker {
  pub fn new(line_body_char: u8) -> AnchoredLineMaker {
    AnchoredLineMaker {
      lines: Vec::new(),
      line_begin: None,
      line_body_char,
    }
  }

  pub fn process(&mut self, pos: &Point, byte: &u8) {
    // Feed a character to the AnchoredLineMaker: this looks for ASCII art lines like '+----+'.-
    // When a '+' is observed and line_begin is None, the current position is recorded.
    // If line begin is set and the current character is the same as line_body_char, the
    // line is extended. If the current character is a '+', the line is closed and added
    // to the list of lines and line_begin is reset to None.
    // If some other character is observed in the middle (e.g., '+---a---+' the attempt
    // to create a line is abandoned (and line_begin becomes None).
    // A Line must contain at least one line_body character ('++' is not a line).

    if let Some(begin) = self.line_begin {
      // in order to ensure that the line is at least one character long, we need to
      // check the distance between the current position and the line begin position:
      if *byte == b'+' && pos.distance(&begin) > 1 {
        let line = AnchoredLine::from_points(&begin, &pos, Both).unwrap();
        println!("new line: {:?}", line);
        self.lines.push(line);
        self.line_begin = None;
        self.process(pos, byte);
      } else if *byte != self.line_body_char {
        self.line_begin = None;
      }
    } else if *byte == b'+' {
      self.line_begin = Some(*pos);
    }
  }
}
