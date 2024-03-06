use crate::connected_line_maker::ConnectedLineMaker;
use crate::simple_geo::ConnectedLine;
use crate::simple_geo::Point;
use crate::simple_geo::Word;
use crate::util::ErrString;
use std::cell::RefCell;
use std::rc::Rc;

/////////////////////////////////////////////////////////////////////////////////
pub fn make_process_bidirectionally_fun<'a>(
  line_body_char: u8,
  wall_char: u8,
  collect_words: bool,
  allow_length_one: bool,
  is_invalid_byte: impl Fn(u8) -> Option<ErrString> + 'a,
  pos_preprocessor: impl Fn(Point) -> Point + 'a,
  line_postprocessor: impl Fn(ConnectedLine) -> ConnectedLine + 'a,
  word_postprocessor: impl Fn(Word) -> Word + 'a,
  custom_printer: impl Fn(Point) + 'a,
) -> (Rc<RefCell<ConnectedLineMaker<'a>>>, impl Fn(&Point, &u8) + 'a) {
  let linemaker = ConnectedLineMaker::new(
    line_body_char,
    wall_char,
    collect_words,
    allow_length_one,
    line_postprocessor,
    word_postprocessor,
  );
  let rc_linemaker = Rc::new(RefCell::new(linemaker));
  let rc_linemaker_twin = Rc::clone(&rc_linemaker);

  (rc_linemaker, move |pos: &Point, byte: &u8| {
    let pos = pos_preprocessor(*pos);

    if let Some(err) = is_invalid_byte(*byte) {
      panic!("{} at {:?}", err, pos);
    }

    custom_printer(pos);
    rc_linemaker_twin.borrow_mut().process(pos, *byte);
  })
}
