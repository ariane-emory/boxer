use crate::simple_geo::ConnectedLine;
use crate::simple_geo::ConnectionType;
use crate::simple_geo::ConnectionType::{Corner, Nothing, Wall};
use crate::simple_geo::Point;
use crate::simple_geo::Word;


////////////////////////////////////////////////////////////////////////////////
fn is_word_char(byte: u8) -> bool {
  const WORD_CHARS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!=*/%_";
  let result = WORD_CHARS.as_bytes().contains(&byte);
  // let msg = if result {
  //   "is word char"
  // }
  // else {
  //   "is not word char"
  // };

  // println!("byte '{}' {}.", byte as char, msg);

  result
}


////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ConnectedLineMaker {
  pub lines: Vec<ConnectedLine>,
  pub words: Vec<Word>,
  line_begin: Option<Point>,
  line_begin_type: ConnectionType,
  line_body_char: u8,
  wall_char: u8,
  collect_words: bool,
  current_word: String,
  prev_pos: Point,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ConnectedLineMaker {
  pub fn new(
    line_body_char: u8,
    wall_char: u8,
    collect_words: bool,
  ) -> ConnectedLineMaker {
    ConnectedLineMaker {
      lines: Vec::new(),
      words: Vec::new(),
      line_begin: None,
      line_begin_type: Corner,
      line_body_char,
      wall_char,
      collect_words,
      current_word: String::new(),
      prev_pos: Point::new(std::usize::MAX, std::usize::MAX),
    }
  }

  fn begin_line(&mut self, pos: Point, connection_type: ConnectionType) {
    self.try_collect_word(pos);
    self.line_begin = Some(pos);
    self.line_begin_type = connection_type;
  }

  fn try_collect_word(&mut self, _pos: Point) {
    if self.collect_words && self.current_word.len() > 0 {
      let word_start = Point::new(
        self.prev_pos.line,
        self.prev_pos.col - self.current_word.len() + 1,
      );
      println!(
        "Pushing word: {:?} @ {:?} → {:?}",
        self.current_word, word_start, self.prev_pos
      );
      self.words.push(
        Word::new(&self.current_word, word_start, self.prev_pos).unwrap(),
      );
      self.current_word = String::new();
    }
    self.current_word = String::new();
  }

  fn reset(&mut self, pos: Point) {
    self.try_collect_word(pos);
    self.line_begin = None;
    self.line_begin_type = Nothing;
  }

  fn complete_line(
    &mut self,
    byte: u8,
    begin: Point,
    end: Point,
    connectection_type: ConnectionType,
  ) {
    let line =
      ConnectedLine::new(begin, end, self.line_begin_type, connectection_type)
        .unwrap();
    println!("         CREATED LINE: {:?}", line);
    self.lines.push(line);
    self.reset(end);
    self.process(end, byte);
  }

  pub fn process(&mut self, pos: Point, byte: u8) {
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
    if pos.line != self.prev_pos.line {
      println!("         new line, abort!");
      self.reset(pos);
    }

    if let Some(begin) = self.line_begin {
      // in order to ensure that the line is at least one character long, we
      // will need to check the distance between the current position and
      // the line begin position:
      let distance_ok = pos.distance(&begin) > 1;

      if byte == b'+' {
        if distance_ok {
          self.complete_line(byte, begin, pos, Corner);
        }
        else {
          self.reset(pos);
          println!("Begin line at Corner after line break!");
          self.begin_line(pos, Corner);
        }
      }
      else if byte == self.wall_char && distance_ok {
        self.complete_line(byte, begin, pos, Wall);
      }
      else if byte != self.line_body_char {
        println!("         broke line, distance = {}!", pos.distance(&begin));
        self.reset(pos);
        self.process(pos, byte);
      }
    }
    else {
      if byte == b'+' {
        println!("Begin line at Corner.");
        self.begin_line(pos, Corner);
      }
      else if byte == self.wall_char {
        println!("Begin line at Wall.");
        self.begin_line(pos, Wall);
      }
      else if self.collect_words && is_word_char(byte) {
        self.current_word.push(byte as char);
        println!(
          "Add char '{}', word = \"{}\".",
          byte as char, self.current_word
        );
      }
    }

    self.prev_pos = pos;
  }
}
