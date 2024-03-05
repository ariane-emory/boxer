mod extract_lines_and_words;
mod make_process_bidirectionally_fun;

//use crate::simple_geo::find_rectangles;
use crate::simple_geo::ConnectedLine;
use crate::simple_geo::LineMethods;
use crate::simple_geo::Offsetable;
use crate::simple_geo::Word;
use crate::simple_matrix::*;
use crate::util::vec_utils::Removeql;
use crate::util::vec_utils::SortedInsert;

pub use extract_lines_and_words::extract_lines_and_words;
pub use make_process_bidirectionally_fun::make_process_bidirectionally_fun;
use std::io::Result;

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
pub fn process_file(path: &str) -> Result<()> {
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
    .for_each(|word| println!("Word:               {:?}", word));

  // println!("");
  // println!("================================================================================");
  // println!("Finding rectangles in:");
  // println!("================================================================================");

  // let mut non_rectangle_candidate_lines: Vec<ConnectedLine> = Vec::new();
  // non_rectangle_candidate_lines
  //   .extend(free_lines.iter().filter(|cl| !cl.is_corner_connected()));

  // let mut rectangle_candidate_lines: Vec<ConnectedLine> = free_lines;
  // rectangle_candidate_lines.retain(ConnectedLine::is_corner_connected);

  // non_rectangle_candidate_lines
  //   .iter()
  //   .for_each(|line| println!("Non-Candidate Line: {:?}", line));
  // rectangle_candidate_lines
  //   .iter()
  //   .for_each(|line| println!("Candidate Line:     {:?}", line));
  // words
  //   .iter()
  //   .for_each(|word| println!("Word:               {:?}", word));

  // println!("");
  // println!("================================================================================");
  // println!("Found:");
  // println!("================================================================================");

  // let (rectangles, mut free_lines) =
  //   find_rectangles(rectangle_candidate_lines, false);

  // free_lines.extend(non_rectangle_candidate_lines);

  // free_lines
  //   .iter()
  //   .for_each(|line| println!("Free Line:          {:?}", line));
  // rectangles
  //   .iter()
  //   .for_each(|rect| println!("Rectangle:          {:?}", rect));
  // words
  //   .iter()
  //   .for_each(|word| println!("Word:               {:?}", word));

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

  println!("");
  println!("================================================================================");
  println!("Looking for chains...");
  println!("================================================================================");
  println!("");

  let chains = ConnectedLine::find_chains(&free_lines);

  for (i, chain) in chains.iter().enumerate() {
    //chain.sort();
    println!("Chain {}: length {} ", i, chain.len());
    chain.iter().for_each(|line| println!("  {:?}", line));

    if let Some((start, end)) = ConnectedLine::analyze_chain(&chain) {
      println!("  Start: {:?}, End: {:?}", start, end);
    }
    else {
      println!("  Chain ends not found.");
    }
  }

  Ok(())
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
