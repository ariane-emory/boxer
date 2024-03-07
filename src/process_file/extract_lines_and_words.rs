use crate::noisy_print;
use crate::process_file::make_process_bidirectionally_fun;
use crate::simple_geo::ConnectedLine;
use crate::simple_geo::Flippable;
use crate::simple_geo::Offsetable;
use crate::simple_geo::Orientation::*;
use crate::simple_geo::Point;
use crate::simple_geo::Word;
use crate::simple_matrix::process_matrix_bidirectionally;
use crate::util::noisy_print;
use crate::util::ErrString;

/////////////////////////////////////////////////////////////////////////////////
static LINE_OFFSET: isize = 1;

/////////////////////////////////////////////////////////////////////////////////
pub fn extract_lines_and_words(
  normalized_matrix: &Vec<Vec<u8>>,
) -> (Vec<ConnectedLine>, Vec<Word>) {
  let mut free_lines = Vec::new();
  let mut words = Vec::new();
  let flip_line = |cl: ConnectedLine| cl.flip();
  let do_nothing_to_line = |line: ConnectedLine| line;
  let do_nothing_to_word = |wrd: Word| wrd;
  let log_fun = |ori, flip_pos| {
    move |pos: Point, byte: u8| {
      let tmp = format!("{:?}", byte as char);
      noisy_print!(
        "\n[{:12?} {:4}@ {:?}] ",
        ori,
        tmp,
        if flip_pos { pos.flip() } else { pos },
      );
      // noisy_print!("\n[{:12?}] ", ori);
    }
  };
  let is_non_ascii_byte = |byte: u8| {
    (byte & 128 != 0)
      .then(|| ErrString::new(&format!("Non-ASCII byte {}", byte)))
  };
  let offset_line = |pos: Point| pos.offset_by(LINE_OFFSET, 0);
  let offset_column = |pos: Point| pos.offset_by(0, LINE_OFFSET);

  let (vert_linemaker, process_vert_fun) = make_process_bidirectionally_fun(
    b'|',
    b'-',
    false,
    is_non_ascii_byte,
    offset_column,
    flip_line,
    do_nothing_to_word,
    log_fun(Vertical, true),
  );

  let (horiz_linemaker, process_horiz_fun) = make_process_bidirectionally_fun(
    b'-',
    b'|',
    true,
    is_non_ascii_byte,
    offset_line,
    do_nothing_to_line,
    do_nothing_to_word,
    log_fun(Horizontal, false),
  );

  process_matrix_bidirectionally(
    normalized_matrix,
    process_horiz_fun,
    process_vert_fun,
  );

  noisy_print!("\n");

  words.extend(horiz_linemaker.borrow().words.iter().cloned());
  free_lines.extend(horiz_linemaker.borrow().lines.iter());
  free_lines.extend(vert_linemaker.borrow().lines.iter());
  free_lines.sort();

  (free_lines, words)
}
