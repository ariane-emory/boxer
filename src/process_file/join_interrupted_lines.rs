use crate::simple_geo::ConnectedLine;
use crate::simple_geo::ConnectionType::*;
use crate::simple_geo::Orientation::*;

////////////////////////////////////////////////////////////////////////////////
pub fn join_interrupted_lines(lines: Vec<ConnectedLine>) -> Vec<ConnectedLine> {
  let horizontal_lines = lines
    .iter()
    .filter(|cl| cl.orientation == Horizontal)
    .cloned()
    .collect::<Vec<ConnectedLine>>();
  let vertical_lines = lines
    .iter()
    .filter(|cl| cl.orientation == Vertical)
    .cloned()
    .collect::<Vec<ConnectedLine>>();
  let horizontal_lines =
    join_similarly_oriented_interrupted_lines(horizontal_lines);
  let vertical_lines =
    join_similarly_oriented_interrupted_lines(vertical_lines);
  let mut lines = Vec::new();
  lines.extend(horizontal_lines);
  lines.extend(vertical_lines);
  lines.sort();
  lines
}

// This function is meant to be passed a Vec of lines whose orientations match:
fn join_similarly_oriented_interrupted_lines(
  mut lines: Vec<ConnectedLine>,
) -> Vec<ConnectedLine> {
  let mut merged_lines: Vec<ConnectedLine> = Vec::new();
  lines.sort();
  lines.reverse();
  while let Some(mut line) = lines.pop() {
    println!("Looking for merges for {:?}...", line);

    while line.end_connects_to == Wall {
      if let Some(other) = lines.pop() {
        if line.end != other.start || other.start_connects_to != Wall {
          break;
        }
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
    }
    merged_lines.push(line);
  }
  merged_lines
}
