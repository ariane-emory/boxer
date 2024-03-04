use crate::line_makers::ConnectedLineMaker;
use crate::simple_geo::find_rectangles;
use crate::simple_geo::ConnectedLine;
use crate::simple_geo::LineMethods;
use crate::simple_geo::Offsetable;
use crate::simple_geo::Orientation;
use crate::simple_geo::Orientation::*;
use crate::simple_geo::Point;
use crate::simple_geo::Rectangle;
use crate::simple_geo::Word;
use crate::simple_matrix::*;

use std::cell::RefCell;
use std::io::Result as IoResult;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////
static LINE_OFFSET: isize = 1;

/////////////////////////////////////////////////////////////////////////////////
pub fn make_process_bidirectionally_fun<'a>(
  orientation: Orientation,
  line_body_char: u8,
  wall_char: u8,
  collect_words: bool,
  allow_length_one: bool,
  pos_preprocessor: impl Fn(Point) -> Point + 'a,
  line_postprocessor: impl Fn(ConnectedLine) -> ConnectedLine + 'a,
  word_postprocessor: impl Fn(Word) -> Word + 'a,
  custom_printer: impl Fn(Orientation, Point, u8) + 'a,
) -> (Rc<RefCell<ConnectedLineMaker<'a>>>, impl Fn(&Point, &u8) + 'a) {
  let linemaker = ConnectedLineMaker::new(
    orientation,
    line_body_char,
    wall_char,
    collect_words,
    allow_length_one,
    //pos_preprocessor,
    line_postprocessor,
    word_postprocessor,
  );
  let rc_linemaker = Rc::new(RefCell::new(linemaker));
  let rc_linemaker_twin = Rc::clone(&rc_linemaker);

  (rc_linemaker, move |pos: &Point, byte: &u8| {
    if pos.col == 0 {
      println!("");
    }

    let pos = pos_preprocessor(*pos);

    if 0 != (*byte & 128) {
      panic!("Found non-ASCII byte {} at {:?}", byte, pos);
    }

    custom_printer(orientation, pos, *byte);
    rc_linemaker_twin.borrow_mut().process(pos, *byte);
  })
}

/////////////////////////////////////////////////////////////////////////////////
fn extract_basic_geometry(
  uniform_matrix: &Vec<Vec<u8>>,
) -> (Vec<Rectangle>, Vec<ConnectedLine>, Vec<Word>) {
  let mut rectangles = Vec::new();
  let mut other_lines = Vec::new();
  let mut words = Vec::new();

  // all_lines scope:
  {
    let mut all_lines: Vec<ConnectedLine> = Vec::new();
    let flip_pos = |pos: Point| pos.flip();
    let flip_line = |cl: ConnectedLine| cl.flip();
    let flip_word = |wrd: Word| wrd.flip();
    let log_labeled_byte = |ori: Orientation, pos: Point, byte: u8| {
      println!("{:12} {:?}: '{}'", format!("{:?}:", ori), pos, byte as char)
    };
    let log_byte_with_orientation =
      |ori, pos, byte| log_labeled_byte(ori, pos, byte);
    let log_byte_with_orientation_and_flipped_pos =
      |ori, pos, byte| log_labeled_byte(ori, flip_pos(pos), byte);

    // RefCell scope:
    {
      let (vert_linemaker, process_vert_fun) = make_process_bidirectionally_fun(
        Vertical,
        b'|',
        b'-',
        false,
        false,
        |pos| pos.offset_by(0, LINE_OFFSET),
        flip_line,
        flip_word,
        log_byte_with_orientation_and_flipped_pos,
      );

      let (horiz_linemaker, process_horiz_fun) =
        make_process_bidirectionally_fun(
          Horizontal,
          b'-',
          b'|',
          true,
          true,
          |pos| pos.offset_by(LINE_OFFSET, 0),
          |line| line,
          |word| word,
          log_byte_with_orientation,
        );

      process_matrix_bidirectionally(
        &uniform_matrix,
        process_horiz_fun,
        process_vert_fun,
      );

      println!("");

      words.extend(horiz_linemaker.borrow().words.iter().cloned());
      all_lines.extend(horiz_linemaker.borrow().lines.iter());
      all_lines.extend(vert_linemaker.borrow().lines.iter());
    } // End of RefCell scope.

    other_lines.extend(all_lines.iter().filter(|cl| !cl.corner_connected()));
    all_lines.retain(ConnectedLine::corner_connected);

    find_rectangles(&all_lines, &mut rectangles, &mut other_lines, false);
  } // End of all_lines scope.

  println!("");

  other_lines
    .iter()
    .for_each(|line| println!("Other line:      {:?}", line));
  words
    .iter()
    .for_each(|word| println!("Found word:      {:?}", word));
  rectangles
    .iter()
    .for_each(|rect| println!("Found rectangle: {:?}", rect));

  (rectangles, other_lines, words)
}

/////////////////////////////////////////////////////////////////////////////////
pub fn process_file(
  path: &str,
) -> IoResult<(Vec<Vec<u8>>, Vec<Rectangle>, Vec<ConnectedLine>, Vec<Word>)> {
  let matrix: Vec<Vec<u8>> = read_file_to_byte_matrix(path)?;
  let uniform_matrix =
    normalize_matrix_width(&matrix, matrix_max_row_len(&matrix), b' ');

  let (rectangles, other_lines, words) =
    extract_basic_geometry(&uniform_matrix);

  let single_length_lines = other_lines
    .iter()
    .filter(|cl| cl.len() == 1)
    .cloned()
    .collect::<Vec<ConnectedLine>>();

  //let combined_words = Vec::new();

  for line in single_length_lines {
    println!("Single length line: {:?}", line);

    let candidate_words = words
      .iter()
      .filter(|word| word.start == line.start.offset_by(0, 1))
      .cloned()
      .collect::<Vec<Word>>();

    for word in candidate_words {
      println!("  Candidate word: {:?}", word);
    }
  }

  Ok((uniform_matrix, rectangles, other_lines, words))
}
