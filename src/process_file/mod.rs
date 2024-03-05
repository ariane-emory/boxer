mod add_null_sentinels_to_normalized_matrix;
mod extract_lines_and_words;
mod make_process_bidirectionally_fun;
mod merge_length_1_lines_with_words;

//use crate::simple_geo::find_rectangles;
use crate::simple_geo::analyze_chain;
use crate::simple_geo::chain_get_network;
use crate::simple_geo::find_chains;
use crate::simple_matrix::matrix_max_row_len;
use crate::simple_matrix::normalize_matrix_width;
use crate::simple_matrix::read_file_to_byte_matrix;
use add_null_sentinels_to_normalized_matrix::add_null_sentinels_to_normalized_matrix;
use extract_lines_and_words::extract_lines_and_words;
use make_process_bidirectionally_fun::make_process_bidirectionally_fun;
use merge_length_1_lines_with_words::merge_length_1_lines_with_words;
use std::io::Result;

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

  let (free_lines, words) = merge_length_1_lines_with_words(free_lines, words);

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

    let net = chain_get_network(&chain);

    for (i, point) in net.iter().enumerate() {
      println!("  Network point {}: {:?}", i, point);
    }
  }

  Ok(())
}
