use crate::simple_geo::ConnectedLine;
use crate::simple_geo::ConnectionType;
use crate::simple_geo::ConnectionType::{AnotherLine, Nothing, Wall};
use crate::simple_geo::Point;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ConnectedLineMaker {
  pub lines: Vec<ConnectedLine>,
  line_begin: Option<Point>,
  line_begin_type: ConnectionType,
  line_body_char: u8,
  wall_char: u8,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl ConnectedLineMaker {
  pub fn new(line_body_char: u8, wall_char: u8) -> ConnectedLineMaker {
    ConnectedLineMaker {
      lines: Vec::new(),
      line_begin: None,
      line_begin_type: AnotherLine,
      line_body_char,
      wall_char,
    }
  }

  fn abort_line(&mut self) {
    self.line_begin = None;
    self.line_begin_type = Nothing;
  }

  fn begin_line(&mut self, point: Point, connection_type: ConnectionType) {
    self.line_begin = Some(point);
    self.line_begin_type = connection_type;
  }

  fn complete_line(&mut self, pos: Point, connectection_type: ConnectionType) {
    let line = ConnectedLine::new(
      self.line_begin.unwrap(),
      pos,
      self.line_begin_type,
      connectection_type,
    )
    .unwrap();
    println!("         CREATE LINE: {:?}", line);
    self.lines.push(line);
    self.abort_line();
  }

  pub fn process(&mut self, pos: &Point, byte: u8) {
    // Feed a character to the ConnectedLineMaker: this looks for ASCII art
    // lines like '+----+'.- When a '+' is observed and line_begin is None,
    // the current position is recorded. If line begin is set and the
    // current character is the same as line_body_char, the
    // line is extended. If the current character is a '+', the line is closed
    // and added to the list of lines and line_begin is reset to None.
    // If some other character is observed in the middle (e.g., '+---a---+' the
    // attempt to create a line is abandoned (and line_begin becomes None).
    // A Line must contain at least one line_body character ('++' is not a
    // line).

    if pos.col == 0 {
      println!("         new line!");
      self.abort_line();
    }

    if let Some(begin) = self.line_begin {
      // in order to ensure that the line is at least one character long, we
      // need to check the distance between the current position and the
      // line begin position:
      let distance_ok = pos.distance(&begin) > 1;

      if byte == b'+' && distance_ok {
        let line =
          ConnectedLine::new(begin, *pos, self.line_begin_type, AnotherLine)
            .unwrap();
        println!("         CREATE LINE: {:?}", line);
        self.lines.push(line);
        self.abort_line();
      } else if byte == self.wall_char && distance_ok {
        let line =
          ConnectedLine::new(begin, *pos, self.line_begin_type, Wall).unwrap();
        println!("         CREATE LINE: {:?}", line);
        self.lines.push(line);
        self.abort_line();
      } else if byte != self.line_body_char {
        println!("         broke line, distance = {}!", pos.distance(&begin));
        self.abort_line();
      }
    } else if byte == b'+' {
      self.begin_line(*pos, AnotherLine);
    } else if byte == self.wall_char {
      self.begin_line(*pos, Wall);
    }
  }
}
