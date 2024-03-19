// -*- c-backslash-column: 95; c-backslash-max-column: 95 ; fill-column: 95; -*-
// #![allow(unused_variables)]
// #![allow(unused_mut)]

mod add_null_sentinels_to_normalized_matrix;
mod check_for_illegal_networks;
mod extract_lines_and_words;
mod join_interrupted_lines;
mod make_process_bidirectionally_fun;
mod merge_length_1_lines_with_words;

use crate::simple_geo::find_networks;
use crate::simple_geo::find_rectangles;
use crate::simple_geo::network_get_endpoints;
use crate::simple_geo::ConnectedLine;
use crate::simple_matrix::matrix_max_row_len;
use crate::simple_matrix::normalize_matrix_width;
use crate::simple_matrix::read_file_to_byte_matrix;
use add_null_sentinels_to_normalized_matrix::add_null_sentinels_to_normalized_matrix;
use check_for_illegal_networks::check_for_illegal_networks;
use extract_lines_and_words::extract_lines_and_words;
use join_interrupted_lines::join_interrupted_lines;
use make_process_bidirectionally_fun::make_process_bidirectionally_fun;
use merge_length_1_lines_with_words::merge_length_1_lines_with_words;
use std::io::Result;

///////////////////////////////////////////////////////////////////////////////////////////////
pub fn process_file(path: &str) -> Result<()> {
  let matrix = read_file_to_byte_matrix(path)?;
  let matrix =
    normalize_matrix_width(&matrix, matrix_max_row_len(&matrix), b' ');
  let matrix = add_null_sentinels_to_normalized_matrix(matrix);

  println!();
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

  println!();
  println!("================================================================================");
  println!("Finding rectangles in:");
  println!("================================================================================");

  let mut non_rectangle_candidate_lines: Vec<ConnectedLine> = Vec::new();
  non_rectangle_candidate_lines
    .extend(free_lines.iter().filter(|cl| !cl.is_corner_connected()));

  let mut rectangle_candidate_lines: Vec<ConnectedLine> = free_lines;
  rectangle_candidate_lines.retain(ConnectedLine::is_corner_connected);

  non_rectangle_candidate_lines
    .iter()
    .for_each(|line| println!("Non-Candidate Line: {:?}", line));
  rectangle_candidate_lines
    .iter()
    .for_each(|line| println!("Candidate Line:     {:?}", line));
  words
    .iter()
    .for_each(|word| println!("Word:               {:?}", word));

  println!();
  println!("================================================================================");
  println!("Found:");
  println!("================================================================================");

  let (rectangles, mut free_lines) =
    find_rectangles(rectangle_candidate_lines, false);

  free_lines.extend(non_rectangle_candidate_lines);
  free_lines.sort();

  free_lines
    .iter()
    .for_each(|line| println!("Free Line:          {:?}", line));
  rectangles
    .iter()
    .for_each(|rect| println!("Rectangle:          {:?}", rect));
  words
    .iter()
    .for_each(|word| println!("Word:               {:?}", word));

  println!();
  println!("================================================================================");
  println!("Finding and trying to merge length-1 lines with Words...");
  println!("================================================================================");

  let (free_lines, words) = merge_length_1_lines_with_words(free_lines, words);

  free_lines
    .iter()
    .for_each(|line| println!("Free Line:          {:?}", line));
  words
    .iter()
    .for_each(|word| println!("Word:               {:?}", word));

  println!();
  println!("================================================================================");
  println!("Merging interrupted lines...");
  println!("================================================================================");

  let free_lines = join_interrupted_lines(free_lines);

  println!();

  free_lines
    .iter()
    .for_each(|line| println!("Free Line:          {:?}", line));

  println!();
  println!("================================================================================");
  println!("Looking for networks...");
  println!("================================================================================");

  let networks = find_networks(free_lines);

  for (i, network) in networks.iter().enumerate() {
    //network.sort();
    println!("Network #{}:", i + 1);
    network.iter().enumerate().for_each(|(i, line)| {
      let str = format!("Line #{}:", i + 1);
      println!("  {:9}      {:?}", str, line);
    });

    let endpoints = network_get_endpoints(network);

    for (i, point) in endpoints.iter().enumerate() {
      println!("  End point #{}:  {:?}", i + 1, point);
    }
  }
  println!();
  println!("=========================-======================================================");
  println!("Checking for illegal networks...");
  println!("================================================================================");
  println!();
  check_for_illegal_networks(&networks, &rectangles);
  println!("No illegal networks detected.");
  Ok(())
}
