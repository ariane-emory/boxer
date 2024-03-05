use crate::simple_geo::line_methods::LineMethods;
use crate::simple_geo::ConnectedLine;
use crate::simple_geo::ConnectionType::*;
use crate::util::RemoveIf;

////////////////////////////////////////////////////////////////////////////////
pub fn merge_interrupted_lines(
  free_lines: Vec<ConnectedLine>,
) -> Vec<ConnectedLine> {
  let mut free_lines = free_lines;
  free_lines.sort();
  free_lines.reverse();

  let mut merged_horizontal_lines: Vec<ConnectedLine> = Vec::new();
  let mut horizontal_lines = free_lines
    .iter()
    .filter(|line| line.is_horizontal())
    .cloned()
    .collect::<Vec<ConnectedLine>>();
  horizontal_lines
    .iter()
    .for_each(|line| println!("Horizontal Line: {:?}", line));

  let mut merged_vertical_lines: Vec<ConnectedLine> = Vec::new();
  let mut vertical_lines = free_lines
    .iter()
    .filter(|line| line.is_vertical())
    .cloned()
    .collect::<Vec<ConnectedLine>>();
  vertical_lines
    .iter()
    .for_each(|line| println!("Vertical Line:   {:?}", line));

  while let Some(mut line) = horizontal_lines.pop() {
    println!("\nLooking for merges for {:?}...", line);

    while let Some(other) = horizontal_lines.remove_if(|o| {
      line.end_connects_to == Wall
        && line.end == o.start
        && o.start_connects_to == Wall
    }) {
      println!("Could merge with {:?}.", other);

      line = ConnectedLine::new(
        line.orientation,
        line.start,
        other.end,
        line.start_connects_to,
        other.end_connects_to,
      )
      .unwrap();

      println!("Merged into: {:?}", line);
    }

    merged_horizontal_lines.push(line);
  }

  println!("");

  merged_horizontal_lines
    .iter()
    .for_each(|line| println!("Merged Horizontal Line: {:?}", line));

  while let Some(mut line) = vertical_lines.pop() {
    println!("\nLooking for merges for {:?}...", line);

    while let Some(other) = vertical_lines.remove_if(|o| {
      line.end_connects_to == Wall
        && line.end == o.start
        && o.start_connects_to == Wall
    }) {
      println!("Could merge with {:?}.", other);

      line = ConnectedLine::new(
        line.orientation,
        line.start,
        other.end,
        line.start_connects_to,
        other.end_connects_to,
      )
      .unwrap();

      println!("Merged into: {:?}", line);
    }

    merged_vertical_lines.push(line);
  }

  println!("");

  merged_vertical_lines
    .iter()
    .for_each(|line| println!("Merged Vertical Line: {:?}", line));

  let mut all_merged_lines = Vec::new();
  all_merged_lines.extend(merged_horizontal_lines);
  all_merged_lines.extend(merged_vertical_lines);
  all_merged_lines.sort();
  all_merged_lines
}
