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
use std::collections::{HashMap, HashSet};
use std::io::Result;
use std::rc::Rc;

/////////////////////////////////////////////////////////////////////////////////
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
pub fn add_null_sentinels_to_normalized_matrix(
  mut matrix: Vec<Vec<u8>>,
) -> Vec<Vec<u8>> {
  let normalized_matrix_width = matrix[0].len();
  let terminator = b'\0';

  for row in matrix.iter_mut() {
    row.push(terminator);
  }

  matrix.push(vec![terminator; normalized_matrix_width + 1]);
  matrix
}

/////////////////////////////////////////////////////////////////////////////////
pub fn process_file(
  path: &str,
) -> Result<(Vec<Vec<u8>>, Vec<Rectangle>, Vec<ConnectedLine>, Vec<Word>)> {
  let matrix = read_file_to_byte_matrix(path)?;
  let matrix =
    normalize_matrix_width(&matrix, matrix_max_row_len(&matrix), b' ');
  let matrix = add_null_sentinels_to_normalized_matrix(matrix);

  println!("");
  println!("================================================================================");
  println!("Extracting ConnectedLines and Words from matrix...");
  println!("================================================================================");

  let (free_lines, words) = extract_lines_and_words(&matrix);


  free_lines
    .iter()
    .for_each(|line| println!("Free Line:          {:?}", line));

  words
    .iter()
    .for_each(|word| println!("Words:              {:?}", word));

  println!("");
  println!("================================================================================");
  println!("Finding rectangles in:");
  println!("================================================================================");

  let mut non_rectangle_candidate_lines: Vec<ConnectedLine> = Vec::new();
  non_rectangle_candidate_lines
    .extend(free_lines.iter().filter(|cl| !cl.corner_connected()));

  let mut rectangle_candidate_lines: Vec<ConnectedLine> = free_lines;
  rectangle_candidate_lines.retain(ConnectedLine::corner_connected);

  non_rectangle_candidate_lines
    .iter()
    .for_each(|line| println!("Non-Candidate Line: {:?}", line));
  rectangle_candidate_lines
    .iter()
    .for_each(|line| println!("Candidate Line:     {:?}", line));

  println!("");
  println!("================================================================================");
  println!("Found:");
  println!("================================================================================");

  let (rectangles, mut free_lines) =
    find_rectangles(rectangle_candidate_lines, false);

  free_lines.extend(non_rectangle_candidate_lines);

  free_lines
    .iter()
    .for_each(|line| println!("Free Line:          {:?}", line));
  words
    .iter()
    .for_each(|word| println!("Word:               {:?}", word));
  rectangles
    .iter()
    .for_each(|rect| println!("Rectangle:          {:?}", rect));

  println!("");
  println!("================================================================================");
  println!("Finding and trying to merge length-1 lines with Words...");
  println!("================================================================================");
  println!("");

  let (free_lines, words) = merge_length_1_lines(free_lines, words);

  free_lines
    .iter()
    .for_each(|line| println!("Free Line:          {:?}", line));
  words
    .iter()
    .for_each(|word| println!("Word:               {:?}", word));
  rectangles
    .iter()
    .for_each(|rect| println!("Rectangle:          {:?}", rect));


  println!("");
  println!("================================================================================");
  println!("Looking for chains...");
  println!("================================================================================");
  println!("");

  let chains = find_chains(&free_lines);

  for (i, chain) in chains.iter().enumerate() {
    //chain.sort();
    println!("Chain {}: length {} ", i, chain.len());
    chain.iter().for_each(|line| println!("  {:?}", line));

    if let Some((start, end)) = analyze_chain(&chain) {
      println!("  Start: {:?}, End: {:?}", start, end);
    }
    else {
      println!("  Chain ends not found.");
    }
  }

  Ok((matrix, rectangles, free_lines, words))
}

////////////////////////////////////////////////////////////////////////////////
fn merge_length_1_lines(
  mut lines: Vec<ConnectedLine>,
  mut words: Vec<Word>,
) -> (Vec<ConnectedLine>, Vec<Word>) {
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

  (lines, words)
}

////////////////////////////////////////////////////////////////////////////////
fn find_chains(lines: &Vec<ConnectedLine>) -> Vec<Vec<ConnectedLine>> {
  let mut graph: HashMap<Point, Vec<ConnectedLine>> = HashMap::new();

  for line in lines {
    graph.entry(line.start).or_default().push(*line);
    graph.entry(line.end).or_default().push(*line);
  }

  let mut visited_points: HashSet<Point> = HashSet::new();
  let mut visited_lines: HashSet<ConnectedLine> = HashSet::new();
  let mut chains: Vec<Vec<ConnectedLine>> = Vec::new();

  fn dfs(
    point: &Point,
    graph: &HashMap<Point, Vec<ConnectedLine>>,
    visited_points: &mut HashSet<Point>,
    visited_lines: &mut HashSet<ConnectedLine>,
    chain: &mut Vec<ConnectedLine>,
  ) {
    if visited_points.insert(*point) {
      if let Some(lines) = graph.get(point) {
        for line in lines {
          if visited_lines.insert(*line) {
            chain.push(*line);
            let next_point = if line.start == *point {
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

////////////////////////////////////////////////////////////////////////////////
fn analyze_chain(
  chain: &Vec<ConnectedLine>,
) -> Option<(Option<Point>, Option<Point>)> {
  let mut point_occurrences = HashMap::new();

  // Count occurrences of each point
  for line in chain {
    *point_occurrences.entry(line.start).or_insert(0) += 1;
    *point_occurrences.entry(line.end).or_insert(0) += 1;
  }

  let mut unique_points = point_occurrences
    .into_iter()
    .filter(|&(_, count)| count == 1)
    .map(|(point, _)| point)
    .collect::<Vec<_>>();

  unique_points.sort();

  let start = unique_points.first().cloned();
  let end = unique_points.last().cloned();

  // If there are exactly 2 unique points, return them as start and end
  // If there are 0 unique points, it's a loop (return None to indicate no
  // unique start/end) If there's 1 or more than 2 unique points, the chain
  // might be malformed or disconnected
  match unique_points.len() {
    2 => Some((start, end)),
    0 => Some((None, None)), // Indicate a loop with None values
    _ => None,               // Malformed or disconnected chain
  }
}

/////////////////////////////////////////////////////////////////////////////////
fn extract_lines_and_words(
  normalized_matrix: &Vec<Vec<u8>>,
) -> (Vec<ConnectedLine>, Vec<Word>) {
  let mut free_lines = Vec::new();
  let mut words = Vec::new();
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

  let (horiz_linemaker, process_horiz_fun) = make_process_bidirectionally_fun(
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

  words.extend(horiz_linemaker.borrow().words.iter().cloned());
  free_lines.extend(horiz_linemaker.borrow().lines.iter());
  free_lines.extend(vert_linemaker.borrow().lines.iter());

  (free_lines, words)
}
