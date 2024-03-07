//#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(unused_variables)]
//#![allow(unreachable_code)]
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
  Something(Point, u8),
  PartialLine(Point, ConnectionType),
  PartialWord(Point, String),
}

////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use ConnectedLineMakerWorkpiece::*;

////////////////////////////////////////////////////////////////////////////////
const ALPHA_CHARS: &str =
  "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NUM_CHARS: &str = "0123456789";
const SYM_CHARS: &str = "[]{}!@#$%^&*()=/_<>:";
const PLUS_MINUS_CHARS: &str = "+-";

////////////////////////////////////////////////////////////////////////////////
fn is_word_char(byte: u8) -> bool {
  false
    || ALPHA_CHARS.as_bytes().contains(&byte)
    || NUM_CHARS.as_bytes().contains(&byte)
    || SYM_CHARS.as_bytes().contains(&byte)
    || PLUS_MINUS_CHARS.as_bytes().contains(&byte)
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
    }
  }

  fn collect_word(&mut self) {
    if !self.collect_words {
      panic!("Inappropriate call to collect_word, not collecting words");
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
      panic!(
        "Inappropriate call to collect_word, invalid workpiece {:?}.",
        self.workpiece
      );
    }
  }

  fn reset(&mut self) {
    noisy_print!("Reset. ");
    self.workpiece = NoWorkpiece;
  }

  fn try_to_complete_line(
    &mut self,
    end: Point,
    end_type: ConnectionType,
    include_current: bool,
    allow_inadequate: bool,
  ) {
    noisy_print!("Complete line: ");
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
    let tmp = format!("{:?}.", byte as char);

    noisy_print!("At {:?} process {:5} ", pos, tmp);

    // Feed a character to the ConnectedLineMaker: this looks for ASCII art
    // lines like '+----+'.- When a '+' is observed and line_begin is None,
    // the current position is recorded. If line begin is set and the
    // current character is the same as bar_char, the
    // line is extended. If the current character is a '+', the line is closed
    // and added to the list of lines and workpiece is reset to NoWorkpiece.
    // If some other character is observed in the middle (e.g., '+---a---+' the
    // attempt to create a line is abandoned (and line_begin becomes None).
    // A Line must contain at least one line_body character ('++' is not a
    // Line).

    match &mut self.workpiece {
      Something(something_begin, something_begin_byte) => {
        match byte {
          // Bar means this Something is actually the start of a Line:
          _ if byte == self.bar_char => {
            noisy_print!("Bar char, beginning line. ");
            self.workpiece = PartialLine(*something_begin, Corner);
          }
          // Word char means this Something is actually the start of a Word:
          _ if is_word_char(byte) => {
            noisy_print!("Word char, beginning word. ");
            self.workpiece = PartialWord(
              *something_begin,
              format!("{}{}", *something_begin_byte as char, byte as char),
            );
          }
          // Whitespace: whatever char started the Something is used as a Word.
          b' ' => {
            self.workpiece = PartialWord(
              *something_begin,
              String::from(format!("{}", *something_begin_byte as char)),
            );
            self.collect_word();
            self.reset();
          }
          // Row terminator:
          b'\0' => {
            // Maybe this case should panic?
            noisy_print!("End of row, reset. ");
            self.reset();
          }
          // Unexpected character:
          _ => self.panic_on_unexpected_char(byte),
        }
      }
      PartialLine(_line_begin, _line_begin_type) => match byte {
        // Bar:
        _ if byte == self.bar_char => {
          noisy_print!("Bar char, continuing line. ")
        }
        // Wall:
        _ if byte == self.wall_char => {
          noisy_print!("Wall, try to complete line. ");
          self.try_to_complete_line(pos, Wall, true, true);
          self.workpiece = PartialLine(pos, Wall);
          noisy_print!("New line begun at {:?}", pos);
        }
        // Corner:
        b'+' => {
          noisy_print!("Corner, try to complete line. ");
          self.try_to_complete_line(pos, Corner, true, true);
          self.workpiece = PartialLine(pos, Corner);
          noisy_print!("New line begun at {:?}", pos);
        }
        // Whitespace:
        b' ' => {
          self.try_to_complete_line(pos, Nothing, false, true);
          self.reset();
        }
        // Word character, treat as whitespace if not collecting words:
        _ if is_word_char(byte) && !self.collect_words => {
          self.try_to_complete_line(pos, Nothing, false, true);
          self.reset();
        }
        // Word character, try to finish the line and begin a word instead if
        // collecting words:
        _ if is_word_char(byte) => {
          noisy_print!("Word char, try to complete line and switch to word. ");
          self.try_to_complete_line(pos, Nothing, false, true);
          self.workpiece =
            PartialWord(pos, String::from(&format!("{}", byte as char)));
        }
        // Row terminator:
        b'\0' => {
          noisy_print!("End of row, line ends in Nothing! ");
          self.try_to_complete_line(pos, Nothing, false, true);
          self.reset();
          //self.process(pos, byte);
        }
        // Unexpected character:
        _ => self.panic_on_unexpected_char(byte),
      },
      PartialWord(_word_begin, ref mut word_string) => {
        match byte {
          // Should never get her if we're not collecting words:
          _ if ! self.collect_words => panic!("Entered PartialWord arm when !collect_words, this should not happen."),
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
            self.collect_word();
            self.reset();
          }
          // Row terminator:
          b'\0' => {
            noisy_print!("End of row, try to complete word. ");
            self.collect_word();
            self.reset();
          }
          // Wall:
          _ if byte == self.wall_char => {
            noisy_print!("Wall, try to complete word. ");
            self.collect_word();
            self.workpiece = PartialLine(pos, Wall);
          }
          // Unexpected character:
          _ => {
            noisy_print!(
              "Looking at {:?}, wall is {:?}",
              byte as char,
              self.wall_char as char,
            );
            self.panic_on_unexpected_char(byte);
          }
        }
      }
      NoWorkpiece => match byte {
        // Bar is definitely a line if we're not collecting words:
        _ if byte == self.bar_char && !self.collect_words => {
          noisy_print!("Begin Line with {:?}. ", byte as char);
          self.workpiece = PartialLine(pos, Wall);
        }
        // Bar is Something if we're collecting words:
        _ if byte == self.bar_char => {
          noisy_print!("Begin Something with {:?}. ", byte as char);
          self.workpiece = Something(pos, byte);
        }
        // Corner is definitely a line if we're not collecting words:
        b'+' if !self.collect_words => {
          noisy_print!("Begin Line with {:?}. ", byte as char);
          self.workpiece = PartialLine(pos, Corner);
        }
        // Corner is Something if we're collecting words:
        b'+' => {
          noisy_print!("Begin Something with {:?}. ", byte as char);
          self.workpiece = Something(pos, byte);
        }
        // Wall is always the start of a line:
        _ if byte == self.wall_char => {
          noisy_print!("Begin Line with {:?}. ", byte as char);
          self.workpiece = PartialLine(pos, Wall);
        }
        // Word char starts a word if we're collecting words:
        _ if is_word_char(byte) && self.collect_words => {
          noisy_print!("Begin Word with {:?}. ", byte as char);
          self.workpiece = PartialWord(pos, format!("{}", byte as char));
        }
        // Word char is ignored when not collecting words:
        _ if is_word_char(byte) => noisy_print!("Word char, ignoring. "),
        // Whitespace is ignored since there's no workpiece:
        b' ' => noisy_print!("Whitespace with no workpiece, do nothing. "),
        // Row terminator is ignored since there's no workpiece:
        b'\0' => noisy_print!("End of row with no workpiece, do nothing. "),
        // Unexpected character, panic.
        _ => self.panic_on_unexpected_char(byte),
      },
    }
  }
}
