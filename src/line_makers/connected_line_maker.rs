use crate::simple_geo::ConnectedLine;
use crate::simple_geo::ConnectionType;
use crate::simple_geo::ConnectionType::{Corner, Nothing, Wall};
use crate::simple_geo::Offsetable;
use crate::simple_geo::Orientation;
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
  orientation: Orientation,
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
  current_word: String,
  current_word_begin: Point,
  prev_pos: Point,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'a> ConnectedLineMaker<'a> {
  pub fn new(
    orientation: Orientation,
    line_body_char: u8,
    wall_char: u8,
    collect_words: bool,
    allow_length_one: bool,
    line_postprocessor: impl Fn(ConnectedLine) -> ConnectedLine + 'a,
    word_postprocessor: impl Fn(Word) -> Word + 'a,
  ) -> ConnectedLineMaker<'a> {
    ConnectedLineMaker {
      orientation,
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
      current_word: String::new(),
      current_word_begin: Point::new(std::usize::MAX, std::usize::MAX),
      prev_pos: Point::new(std::usize::MAX, std::usize::MAX),
    }
  }

  fn try_collect_word(&mut self) {
    if self.collect_words && self.current_word.len() > 0 {
      self.words.push((self.word_postprocessor)(
        Word::new(
          &self.current_word,
          self.current_word_begin,
          self
            .current_word_begin
            .offset_by(0, (self.current_word.len()) as isize),
        )
        .unwrap(),
      ))
    }
    self.current_word = String::new();
  }

  fn reset(&mut self) {
    self.try_collect_word();
    self.line_begin = None;
    self.line_begin_type = Nothing;
  }

  fn begin_line(&mut self, pos: Point, connection_type: ConnectionType) {
    println!("Begin line at {:?}.", connection_type);
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
  ) {
    let line = ConnectedLine::new(
      self.orientation,
      begin,
      end,
      self.line_begin_type,
      connectection_type,
    )
    .unwrap();
    println!("         CREATED LINE: {:?}", line);
    self.lines.push((self.line_postprocessor)(line));
    self.reset();
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
      self.reset();
    }

    if let Some(begin) = self.line_begin {
      // in order to ensure that the line is at least two characters long, we
      // will need to check the distance between the current position and
      // the line begin position:
      let distance_ok = pos.distance(&begin) > 1
        || (self.line_begin_type == Nothing && self.allow_length_one);

      if byte == b'+' {
        if distance_ok {
          self.complete_line(byte, begin, pos, Corner);
        }
        else {
          println!("Begin line at Corner after line break!");
          self.begin_line(pos, Corner);
        }
      }
      else if byte == self.wall_char && distance_ok {
        self.complete_line(byte, begin, pos, Wall);
      }
      else if byte != self.line_body_char {
        if distance_ok {
          self.complete_line(byte, begin, pos.offset_by(0, -1), Nothing);
        }
        else {
          println!("         broke line, distance = {}!", pos.distance(&begin));
          self.reset();
          self.process(pos, byte);
        }
      }
    }
    else {
      if byte == b'+' {
        self.begin_line(pos, Corner);
      }
      if byte == self.line_body_char {
        self.begin_line(pos, Nothing);
      }
      else if byte == self.wall_char {
        self.begin_line(pos, Wall);
      }
      else if self.collect_words && is_word_char(byte) {
        if self.current_word.len() == 0 {
          self.current_word_begin = pos;
        }
        self.current_word.push(byte as char);
        println!(
          "Add char '{}', word = \"{}\".",
          byte as char, self.current_word
        );
      }
      else if byte == b' ' {
        println!(
          "Reset for '{}',      holding {:?}.",
          byte as char, self.current_word
        );
        self.reset();
      }
      else {
        println!(
          "Do nothing for '{}', holding {:?}.",
          byte as char, self.current_word
        );
      }
    }

    self.prev_pos = pos;
  }
}
