// -*- fill-column: 80; eval: (display-fill-column-indicator-mode 1); -*-
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
use crate::simple_geo::Point;
use crate::simple_geo::Word;
use crate::util::noisy_print;
use crate::util::noisy_println;

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum ConnectedLineMakerWorkpiece {
  NoWorkpiece,
  Something(Point, u8),
  PartialLine(Point, ConnectionType),
  PartialWord(Point, String),
}

////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use ConnectedLineMakerWorkpiece::*;

////////////////////////////////////////////////////////////////////////////////
fn is_word_char(byte: u8) -> bool {
  const WORD_CHARS: &str =
  "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789[]{}!@#$%^&*()=/_<>:+-";
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

  fn try_to_collect_word(&mut self) {
    if !self.collect_words {
      return;
    }
    if let PartialWord(word_begin, word_string) = &self.workpiece {
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
      panic!("Inappropriate call to try_to_collect_word");
    }
  }

  // fn reset(&mut self) {
  //   //self.try_to_collect_word();
  //       noisy_print!("Reset. ");
  // }

  fn begin_line(&mut self, pos: Point, connection_type: ConnectionType) {
    noisy_print!("Begin line with {:?} at {:?}. ", connection_type, pos);
    self.workpiece = PartialLine(pos, connection_type);
  }

  fn begin_something(&mut self, pos: Point, byte: u8) {
    noisy_print!("Begin something with {:?} at {:?}. ", byte as char, pos);
    self.workpiece = Something(pos, byte);
  }

  fn begin_word(&mut self, pos: Point, byte: u8) {
    noisy_print!("Begin word with '{}' at {:?}. ", byte as char, pos);
    let mut string = String::new();
    string.push(byte as char);
    self.workpiece = PartialWord(pos, string);
  }

  fn try_to_complete_line(
    &mut self,
    byte: u8,
    end: Point,
    end_type: ConnectionType,
    include_current: bool,
    allow_inadequate: bool,
  ) {
    noisy_print!("Try to complete line... ");

    if let PartialLine(begin, begin_type) = self.workpiece {
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
      }
      else if allow_inadequate {
        noisy_print!(
          "inadequate line distance {} from {:?} to {:?} permitted. ",
          distance,
          begin,
          end
        );
      }
      else {
        panic!(
          "inadequate line distance {} from {:?} to {:?} not permitted here!",
          distance, begin, end
        );
      }
    }
    else {
      panic!("Inappropriate call to try_to_complete_line");
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

    match &mut self.workpiece {
      Something(something_begin, something_begin_byte) => {
        match byte {
          // Bar means this 'something' is actually the start of a Line:
          _ if byte == self.bar_char => {
            noisy_print!("Bar char, beginning line. ");
            self.workpiece = PartialLine(*something_begin, Corner);
          }
          // Word char means this something is actually the start of a Word:
          _ if is_word_char(byte) => {
            noisy_print!("Word char, beginning word. ");
            self.workpiece = PartialWord(
              *something_begin,
              String::from(&format!(
                "{}{}",
                *something_begin_byte as char, byte as char
              )),
            );
          }
          // Whitespace:
          b' ' => {
            self.workpiece = PartialWord(*something_begin, String::from("+"));
            self.try_to_collect_word();
            self.workpiece = NoWorkpiece;
          }
          // Row terminator:
          b'\0' => {
            noisy_print!("End of row, reset. ");
            self.workpiece = NoWorkpiece;
          }
          // Unexpected character:
          _ => self.panic_on_unexpected_char(byte),
        }
      }
      PartialLine(line_begin, line_begin_type) => {
        let distance = pos.distance(line_begin);
        match byte {
          // Bar:
          _ if byte == self.bar_char => {
            noisy_print!("Bar char, continuing line. ")
          }
          // Wall:
          _ if byte == self.wall_char => {
            self.try_to_complete_line(byte, pos, Wall, true, true);
            //self.reset();
            //self.process(pos, byte);
            self.workpiece = PartialLine(pos, Wall);
          }
          // Corner:
          b'+' => {
            noisy_print!("Corner, try to complete line. ");
            self.try_to_complete_line(byte, pos, Corner, true, true);
            //self.reset();
            //self.process(pos, byte);
            println!("New line begun at {:?}", pos);
            self.workpiece = PartialLine(pos, Corner);
          }
          // Whitespace:
          b' ' => {
            self.try_to_complete_line(byte, pos, Nothing, false, true);
            self.workpiece = NoWorkpiece;
            //self.process(pos, byte);
          }
          // Word character, try to finish the line and begin a word instead:
          _ if is_word_char(byte) => {
            noisy_println!(
              "Word char, try to complete line and switch to word. "
            );
            self.try_to_complete_line(byte, pos, Nothing, false, true);
            self.workpiece =
              PartialWord(pos, String::from(&format!("{}", byte as char)));
          }
          // Row terminator:
          b'\0' => {
            noisy_print!("End of row, line ends in Nothing! ");
            self.try_to_complete_line(byte, pos, Nothing, false, true);
            self.workpiece = NoWorkpiece;
          }
          // Unexpected character:
          _ => self.panic_on_unexpected_char(byte),
        }
      }
      PartialWord(word_begin, ref mut word_string) => {
        match byte {
          // Word char':
          _ if is_word_char(byte) => {
            word_string.push(byte as char);
            noisy_print!(
              "Word char, continuing word with '{}': {:?}. ",
              byte as char,
              word_string
            );
          }
          // Whitespace:
          b' ' => {
            noisy_print!("Whitespace, try to complete word. ");
            self.try_to_collect_word();
            self.workpiece = NoWorkpiece;
          }
          // Row terminator:
          b'\0' => {
            noisy_print!("End of row, try to complete word. ");
            self.try_to_collect_word();
            self.workpiece = NoWorkpiece;
          }
          // Wall:
          _ if byte == self.wall_char => {
            noisy_print!("Wall, try to complete word. ");
            self.try_to_collect_word();
            self.workpiece = PartialLine(pos, Wall);
          }
          // Unexpected character:
          _ => self.panic_on_unexpected_char(byte),
        }
      }
      NoWorkpiece => match byte {
        // Bar:
        _ if byte == self.bar_char => {
          if self.collect_words {
            self.begin_something(pos, byte);
          }
          else {
            self.begin_line(pos, Nothing);
          }
        }
        // Corner:
        b'+' => {
          if self.collect_words {
            self.begin_something(pos, byte);
          }
          else {
            self.begin_line(pos, Corner);
          }
        }
        // Wall:
        _ if byte == self.wall_char => self.begin_line(pos, Wall),
        // Word char' when collecting words:
        _ if is_word_char(byte) && self.collect_words => {
          noisy_print!("Word char, begin word with '{}'. ", byte as char);
          self.begin_word(pos, byte)
        }
        // Word char' when not collecting words:
        _ if is_word_char(byte) => {
          noisy_print!("Word char, ignoring. ")
        }
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
