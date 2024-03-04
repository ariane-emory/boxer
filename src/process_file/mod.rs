use crate::noisy_print;
use crate::util::noisy_print;
//use crate::noisy_println;
//use crate::util::noisy_println;

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
use crate::util::vec_utils::Removeql;
use crate::util::vec_utils::SortedInsert;
use std::cell::RefCell;
use std::io::Result as IoResult;
use std::rc::Rc;

/////////////////////////////////////////////////////////////////////////////////
static LINE_OFFSET: isize = 1;

/////////////////////////////////////////////////////////////////////////////////
pub fn process_file(
  path: &str,
) -> IoResult<(Vec<Vec<u8>>, Vec<Rectangle>, Vec<ConnectedLine>, Vec<Word>)> {
  let matrix: Vec<Vec<u8>> = read_file_to_byte_matrix(path)?;
  let mut normalized_matrix =
    normalize_matrix_width(&matrix, matrix_max_row_len(&matrix), b' ');
  let normalize_matrix_width = normalized_matrix[0].len();
  let terminator = b'\0';

  for row in normalized_matrix.iter_mut() {
    row.push(terminator);
  }

  normalized_matrix.push(vec![terminator; normalize_matrix_width + 1]);

  println!("");
  println!("================================================================================");
  println!("Extracting basic geometry...");
  println!("================================================================================");

  let (rectangles, mut lines, mut words) =
    extract_basic_geometry(&normalized_matrix);

  println!("");
  println!("================================================================================");
  println!("Try to merge length-1 lines...");
  println!("================================================================================");
  println!("");

  merge_length_1_lines(&mut lines, &mut words);

  let chains = find_chains(&lines);

  for (i, chain) in chains.iter().enumerate() {
    println!("Chain {}: length {} ", i, chain.len());
    chain.iter().for_each(|line| println!("  {:?}", line));
  }

  Ok((normalized_matrix, rectangles, lines, words))
}

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
    line_postprocessor,
    word_postprocessor,
  );
  let rc_linemaker = Rc::new(RefCell::new(linemaker));
  let rc_linemaker_twin = Rc::clone(&rc_linemaker);

  (rc_linemaker, move |pos: &Point, byte: &u8| {
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
  normalized_matrix: &Vec<Vec<u8>>,
) -> (Vec<Rectangle>, Vec<ConnectedLine>, Vec<Word>) {
  let mut rectangles = Vec::new();
  let mut lines = Vec::new();
  let mut words = Vec::new();

  // all_lines scope:
  {
    let mut all_lines: Vec<ConnectedLine> = Vec::new();
    let flip_pos = |pos: Point| pos.flip();
    let flip_line = |cl: ConnectedLine| cl.flip();
    let flip_word = |wrd: Word| wrd.flip();
    let log_labeled_byte = |ori: Orientation, _pos: Point, _byte: u8| {
      noisy_print!("\n[{:12?}] ", ori);
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
        &normalized_matrix,
        process_horiz_fun,
        process_vert_fun,
      );

      //println!("");

      words.extend(horiz_linemaker.borrow().words.iter().cloned());
      all_lines.extend(horiz_linemaker.borrow().lines.iter());
      all_lines.extend(vert_linemaker.borrow().lines.iter());
    } // End of RefCell scope.

    lines.extend(all_lines.iter().filter(|cl| !cl.corner_connected()));
    all_lines.retain(ConnectedLine::corner_connected);

    find_rectangles(&all_lines, &mut rectangles, &mut lines, false);
  } // End of all_lines scope.

  //println!("");

  lines
    .iter()
    .for_each(|line| println!("Line:            {:?}", line));

  words
    .iter()
    .for_each(|word| println!("Found word:      {:?}", word));

  rectangles
    .iter()
    .for_each(|rect| println!("Found rectangle: {:?}", rect));

  (rectangles, lines, words)
}

////////////////////////////////////////////////////////////////////////////////
fn merge_length_1_lines(lines: &mut Vec<ConnectedLine>, words: &mut Vec<Word>) {
  let single_length_lines = lines
    .iter()
    .filter(|cl| cl.len() == 1)
    .cloned()
    .collect::<Vec<ConnectedLine>>();

  for line in single_length_lines {
    println!("Length 1 line: {:?}", line);

    let candidate_words = words
      .iter()
      .filter(|word| word.start == line.start.offset_by(0, 1))
      .cloned()
      .collect::<Vec<Word>>();

    if candidate_words.len() == 0 {
      panic!("No match for {:?} found.", line);
    }
    else if candidate_words.len() > 1 {
      panic!("Bad data, found more than one candidate word for {:?}", line);
    }
    else {
      println!("  Candidate word: {:?}", candidate_words[0]);

      let new_word = Word::new(
        &format!("-{}", candidate_words[0].string),
        line.start,
        candidate_words[0].end,
      )
      .unwrap();

      println!("  New word: {:?}", new_word);

      lines.removeql(&line);
      words.removeql(&candidate_words[0]);
      words.sorted_insert(new_word);
    }
  }

  println!("");

  lines
    .iter()
    .for_each(|line| println!("Line:      {:?}", line));

  words.iter().for_each(|word| println!("{:?}", word));
}

////////////////////////////////////////////////////////////////////////////////
fn find_chains(lines: &Vec<ConnectedLine>) -> Vec<Vec<ConnectedLine>> {
  use std::collections::{HashMap, HashSet};

  // Graph construction: Map points to lines
  let mut graph: HashMap<Point, Vec<ConnectedLine>> = HashMap::new();
  for line in lines {
    graph
      .entry(line.start.clone())
      .or_default()
      .push(line.clone());
    graph
      .entry(line.end.clone())
      .or_default()
      .push(line.clone());
  }

  let mut visited_points: HashSet<Point> = HashSet::new();
  let mut visited_lines: HashSet<ConnectedLine> = HashSet::new();
  let mut chains: Vec<Vec<ConnectedLine>> = Vec::new();

  // DFS function to explore chains
  fn dfs(
    point: &Point,
    graph: &HashMap<Point, Vec<ConnectedLine>>,
    visited_points: &mut HashSet<Point>,
    visited_lines: &mut HashSet<ConnectedLine>,
    chain: &mut Vec<ConnectedLine>,
  ) {
    if visited_points.insert(point.clone()) {
      if let Some(lines) = graph.get(point) {
        for line in lines {
          if visited_lines.insert(line.clone()) {
            chain.push(line.clone());
            let next_point = if &line.start == point {
              &line.end
            }
            else {
              &line.start
            };
            dfs(next_point, graph, visited_points, visited_lines, chain);
          }
        }
      }
    }
  }

  // Find and group chains
  for point in graph.keys() {
    if !visited_points.contains(point) {
      let mut chain = Vec::new();
      dfs(point, &graph, &mut visited_points, &mut visited_lines, &mut chain);
      if !chain.is_empty() {
        chains.push(chain);
      }
    }
  }

  chains
}
