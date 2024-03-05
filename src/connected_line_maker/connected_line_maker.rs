use crate::noisy_print;
use crate::noisy_println;
use crate::simple_geo::ConnectedLine;
use crate::simple_geo::ConnectionType;
use crate::simple_geo::ConnectionType::{Corner, Nothing, AnotherLine};
use crate::simple_geo::Offsetable;
use crate::simple_geo::Orientation::Horizontal;
use crate::util::noisy_print;
use crate::util::noisy_println;
//use crate::simple_geo::Orientation::*;
use crate::simple_geo::Point;
use crate::simple_geo::Word;

////////////////////////////////////////////////////////////////////////////////
fn is_word_char(byte: u8) -> bool {
  const WORD_CHARS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789[]{}!@#$%^&*()=/_<>:";
  WORD_CHARS.as_bytes().contains(&byte)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ConnectedLineMaker<'a> {
  line_body_char: u8,
  wall_char: u8,
  collect_words: bool,
  allow_length_one: bool,
  line_postprocessor: Box<dyn Fn(ConnectedLine) -> ConnectedLine + 'a>,
  word_postprocessor: Box<dyn Fn(Word) -> Word + 'a>,
  pub lines: Vec<ConnectedLine>,
  pub words: Vec<Word>,
  line_begin: Option<Point>,
  line_begin_type: ConnectionType,
  current_word_begin: Option<Point>,
  current_word: String,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> ConnectedLineMaker<'a> {
  pub fn new(
    line_body_char: u8,
    wall_char: u8,
    collect_words: bool,
    allow_length_one: bool,
    line_postprocessor: impl Fn(ConnectedLine) -> ConnectedLine + 'a,
    word_postprocessor: impl Fn(Word) -> Word + 'a,
  ) -> ConnectedLineMaker<'a> {
    ConnectedLineMaker {
      line_body_char,
      wall_char,
      collect_words,
      allow_length_one,
      line_postprocessor: Box::new(line_postprocessor),
      word_postprocessor: Box::new(word_postprocessor),
      lines: Vec::new(),
      words: Vec::new(),
      line_begin: None,
      line_begin_type: Corner,
      current_word_begin: None,
      current_word: String::new(),
    }
  }

  fn try_collect_word(&mut self) {
    if self.collect_words {
      if let Some(word_begin) = self.current_word_begin {
        if self.current_word.len() > 0 {
          let word = (self.word_postprocessor)(
            Word::new(
              &self.current_word,
              word_begin,
              word_begin.offset_by(0, (self.current_word.len() - 1) as isize),
            )
            .unwrap(),
          );
          noisy_print!("Pushing word {:?}.", word);
          self.words.push(word)
        }
      }
      self.current_word_begin = None;
      self.current_word = String::new();
    }
  }

  fn reset(&mut self) {
    self.try_collect_word();
    self.line_begin = None;
    self.line_begin_type = Nothing;
    noisy_print!("Reset. ");
  }

  fn begin_line(&mut self, pos: Point, connection_type: ConnectionType) {
    noisy_print!("Begin line at {:?}. ", connection_type);
    self.try_collect_word();
    self.line_begin = Some(pos);
    self.line_begin_type = connection_type;
  }

  fn complete_line(
    &mut self,
    byte: u8,
    begin: Point,
    end: Point,
    connectection_type: ConnectionType,
    include_current: bool,
  ) {
    let line_end = if include_current {
      end
    }
    else {
      end.offset_by(0, -1)
    };

    let line = ConnectedLine::new(
      Horizontal,
      begin,
      line_end,
      self.line_begin_type,
      connectection_type,
    )
    .unwrap();

    noisy_print!("Created line {:?}. ", line);

    self.lines.push((self.line_postprocessor)(line));
    self.reset();
    self.process(end, byte);
  }

  pub fn process(&mut self, pos: Point, byte: u8) {
    let tmp = format!("{:?}. ", byte as char);
    noisy_print!("At {:?} process {:6}", pos, tmp);

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

    if let Some(begin) = self.line_begin {
      // in order to ensure that the line is at least two characters long, we
      // will need to check the distance between the current position and
      // the line begin position:
      let distance_ok = pos.distance(&begin) > 1
        || (self.line_begin_type == Nothing && self.allow_length_one);

      if byte == b'\0' {
        if distance_ok {
          noisy_print!("End of row, line ends in Nothing! ");
          self.complete_line(byte, begin, pos, Nothing, false);
        }
        else {
          noisy_print!("End of row, no line! ");
          self.reset();
        }
        noisy_println!("");
      }
      else if byte == b'+' {
        if distance_ok {
          self.complete_line(byte, begin, pos, Corner, true);
        }
        else {
          noisy_print!("Begin line at Corner after line break! ");
          self.begin_line(pos, Corner);
        }
      }
      else if byte == self.wall_char && distance_ok {
        self.complete_line(byte, begin, pos, AnotherLine, true);
      }
      else if byte != self.line_body_char {
        noisy_print!("Broke line, distance = {}. ", pos.distance(&begin));
        if distance_ok {
          self.complete_line(byte, begin, pos, Nothing, false);
        }
        else {
          self.reset();
          self.process(pos, byte);
        }
      }
    }
    else {
      if byte == b'\0' {
        noisy_print!("End of row! ");
        self.reset();
        noisy_println!("");
      }
      if byte == b'+' {
        self.begin_line(pos, Corner);
      }
      if byte == self.line_body_char {
        self.begin_line(pos, Nothing);
      }
      else if byte == self.wall_char {
        self.begin_line(pos, AnotherLine);
      }
      else if self.collect_words && is_word_char(byte) {
        if self.current_word.len() == 0 {
          noisy_print!("Begin word at {:?} with '{}'.", pos, byte as char);
          self.current_word_begin = Some(pos);
        }
        self.current_word.push(byte as char);
        noisy_print!(
          "Add char '{}', word = \"{}\".",
          byte as char,
          self.current_word
        );
      }
      else if byte == b' ' {
        noisy_print!("Whitespace");
        if self.current_word.len() > 0 {
          noisy_print!(", holding {:?}. ", self.current_word);
        }
        else {
          noisy_print!(". ");
        }
        self.reset();
      }
      else {
        noisy_print!("Ignore '{}'", byte as char);
        if self.current_word.len() > 0 {
          noisy_print!(", holding {:?}. ", self.current_word);
        }
        else {
          noisy_print!(". ");
        }
      }
    }
  }
}
