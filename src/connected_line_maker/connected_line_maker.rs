#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
use crate::noisy_print;
use crate::noisy_println;
use crate::simple_geo::ConnectedLine;
use crate::simple_geo::ConnectionType;
use crate::simple_geo::ConnectionType::{Corner, Nothing, Wall};
use crate::simple_geo::Offsetable;
use crate::simple_geo::Orientation::Horizontal;
use crate::util::noisy_print;
use crate::util::noisy_println;
//use crate::simple_geo::Orientation::*;
use crate::simple_geo::Point;
use crate::simple_geo::Word;

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum ConnectedLineMakerWorkpiece {
  NoWorkpiece,
  //SomethingBeginningAtWith(Point, u8),
  LineBeginningAtWith(Point, ConnectionType),
  WordBeginingAtWith(Point, String),
}

////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use ConnectedLineMakerWorkpiece::*;

////////////////////////////////////////////////////////////////////////////////
fn is_word_char(byte: u8) -> bool {
  const WORD_CHARS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789[]{}!@#$%^&*()=/_<>:";
  WORD_CHARS.as_bytes().contains(&byte)
}

////////////////////////////////////////////////////////////////////////////////
pub struct ConnectedLineMaker<'a> {
  bar_char: u8,
  wall_char: u8,
  collect_words: bool,
  allow_length_one: bool,
  line_postprocessor: Box<dyn Fn(ConnectedLine) -> ConnectedLine + 'a>,
  word_postprocessor: Box<dyn Fn(Word) -> Word + 'a>,
  pub lines: Vec<ConnectedLine>,
  pub words: Vec<Word>,
  workpiece: ConnectedLineMakerWorkpiece,
  //line_begin: Option<Point>,
  //line_begin_type: ConnectionType,
  //current_word_begin: Option<Point>,
  //current_word: String,
}
////////////////////////////////////////////////////////////////////////////////
impl<'a> ConnectedLineMaker<'a> {
  pub fn new(
    bar_char: u8,
    wall_char: u8,
    collect_words: bool,
    allow_length_one: bool,
    line_postprocessor: impl Fn(ConnectedLine) -> ConnectedLine + 'a,
    word_postprocessor: impl Fn(Word) -> Word + 'a,
  ) -> ConnectedLineMaker<'a> {
    ConnectedLineMaker {
      bar_char,
      wall_char,
      collect_words,
      allow_length_one,
      line_postprocessor: Box::new(line_postprocessor),
      word_postprocessor: Box::new(word_postprocessor),
      lines: Vec::new(),
      words: Vec::new(),
      workpiece: NoWorkpiece,
      // line_begin: None,
      // line_begin_type: Corner,
      // current_word_begin: None,
      // current_word: String::new(),
    }
  }

  // #[derive(Debug)]
  // enum Dummy {
  //   Nothing,
  //   Something(String, usize),
  // }
  // use Dummy::*;

  // fn main() {
  //   let mut dummy = Something(String::from("hello"), 1);

  //   if let Something(ref mut field, ref mut counter) = dummy {
  //     field.push('!');
  //     *counter += 1;
  //   }

  //   println!("{:?}", dummy);

  //   ()
  // }

  fn try_collect_word(&mut self) {
    if !self.collect_words {
      return;
    }
    if let WordBeginingAtWith(word_begin, word_string) = &self.workpiece {
      if word_string.len() > 0 {
        let word = (self.word_postprocessor)(
          Word::new(
            &word_string,
            *word_begin,
            word_begin.offset_by(0, (word_string.len() - 1) as isize),
          )
          .unwrap(),
        );
        noisy_println!("Pushing word {:?}.", word);
        self.words.push(word)
      }
    }
    else {
      panic!("Inappropriate call to try_collect_word");
    }
  }

  fn reset(&mut self) {
    //self.try_collect_word();
    self.workpiece = NoWorkpiece;
    noisy_print!("Reset. ");
  }

  fn begin_line(&mut self, pos: Point, connection_type: ConnectionType) {
    noisy_print!("Begin line with {:?} at {:?}. ", connection_type, pos);
    self.workpiece = LineBeginningAtWith(pos, connection_type);
  }

  fn begin_word(&mut self, pos: Point, byte: u8) {
    noisy_print!("Begin word with '{}' at {:?}. ", byte as char, pos);
    let mut string = String::new();
    string.push(byte as char);
    self.workpiece = WordBeginingAtWith(pos, string);
  }

  fn try_to_complete_line(
    &mut self,
    byte: u8,
    end: Point,
    end_type: ConnectionType,
    include_current: bool,
  ) {
    noisy_print!("Try to complete line... ");

    if let LineBeginningAtWith(begin, begin_type) = self.workpiece {
      let end = if include_current {
        end
      }
      else {
        end.offset_by(0, -1)
      };
      let distance = end.distance(&begin);
      let distance_ok =
        distance > 1 || (begin_type == Nothing && self.allow_length_one);

      if distance_ok {
        let line =
          ConnectedLine::new(Horizontal, begin, end, begin_type, end_type)
            .unwrap();
        noisy_print!("created line {:?}. ", line);

        let line = (self.line_postprocessor)(line);
        self.lines.push(line);
        // noisy_print!("Pushed line {:?}. ", line);
        noisy_print!("Pushed line. ");

        self.reset();
        self.process(end, byte);
      }
      else {
        noisy_print!(
          "inadequate distance {} from {:?} to {:?}, discarding. ",
          distance,
          begin,
          end
        );
        self.reset();
      }
    }
    else {
      panic!("Confusion in try_to_complete_line");
    }
  }

  fn panic_on_unexpected_char(&self, byte: u8) {
    panic!("Unexpected characer {:?} during {:?}", byte as char, self.workpiece)
  }

  pub fn process(&mut self, pos: Point, byte: u8) {
    noisy_print!("At {:?} process {:?}. ", pos, byte as char);

    // Feed a character to the ConnectedLineMaker: this looks for ASCII art
    // lines like '+----+'.- When a '+' is observed and line_begin is None,
    // the current position is recorded. If line begin is set and the
    // current character is the same as bar_char, the
    // line is extended. If the current character is a '+', the line is closed
    // and added to the list of lines and line_begin is reset to None.
    // If some other character is observed in the middle (e.g., '+---a---+' the
    // attempt to create a line is abandoned (and line_begin becomes None).
    // A Line must contain at least one line_body character ('++' is not a
    // line).

    match &self.workpiece {
      LineBeginningAtWith(line_begin, line_begin_type) => match byte {
        // Bar:
        _ if byte == self.bar_char => {
          noisy_print!("Bar char, continuing line. ")
        }
        // Wall:
        _ if byte == self.wall_char => {
          self.try_to_complete_line(byte, pos, Wall, true)
        }
        // Corner:
        b'+' => {
          noisy_print!("Corner, try to complete line. ");
          self.try_to_complete_line(byte, pos, Corner, true);
        }
        // Whitespace:
        b' ' => self.try_to_complete_line(byte, pos, Nothing, false),
        // Row terminator:
        b'\0' => {
          noisy_print!("End of row, line ends in Nothing! ");
          self.try_to_complete_line(byte, pos, Nothing, false);
        }
        // Unexpected character:
        _ => self.panic_on_unexpected_char(byte),
      },
      WordBeginingAtWith(word_begin, word_string) => {
        panic!("Unhandled case 2: {:?}", self.workpiece)
      }
      NoWorkpiece => match byte {
        // Bar:
        _ if byte == self.bar_char => self.begin_line(pos, Nothing),
        // Wall:
        _ if byte == self.wall_char => self.begin_line(pos, Wall),
        // Word char':
        _ if is_word_char(byte) && self.collect_words => {
          noisy_print!("Word char, begin word with '{}'. ", byte as char);
          self.begin_word(pos, byte)
        }
        _ if is_word_char(byte) => {
          noisy_print!("Word char, ignoring. ")
        }
        // Corner:
        b'+' => self.begin_line(pos, Corner),
        // Whitespace:
        b' ' => noisy_print!("Whitespace with no workpiece, do nothing. "),
        // Row terminator:
        b'\0' => noisy_print!("End of row with no workpiece, do nothing. "),
        // Unexpected character.
        _ => self.panic_on_unexpected_char(byte),
      },
    }
  }
}
