use crate::simple_geo::ConnectedLine;
use crate::simple_geo::ConnectionType::*;
use crate::simple_geo::Orientation::*;
use crate::util::vec_utils::RemoveIf;

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
  println!("--------------------------------------------------------------------------------");
  let vertical_lines =
    join_similarly_oriented_interrupted_lines(vertical_lines);
  let mut lines = Vec::new();
  lines.extend(horizontal_lines);
  lines.extend(vertical_lines);
  lines.sort();
  lines
}

////////////////////////////////////////////////////////////////////////////////
// This function is meant to be passed a Vec of lines whose orientations match:
#[allow(dead_code)]
pub fn good_join_similarly_oriented_interrupted_lines(
  mut lines: Vec<ConnectedLine>,
) -> Vec<ConnectedLine> {
  let mut merged_lines: Vec<ConnectedLine> = Vec::new();
  lines.sort();
  lines.reverse();
  while let Some(mut line) = lines.pop() {
    println!("\nLooking for merges for {:?}...", line);

    while let Some(other) = lines.remove_if(|o| {
      line.end_connects_to == Wall
        && line.orientation == o.orientation
        && line.end == o.start
        && o.start_connects_to == Wall
    }) {
      println!("  Will merge with {:?}.", other);

      line = ConnectedLine::new(
        line.orientation,
        line.start,
        other.end,
        line.start_connects_to,
        other.end_connects_to,
      )
      .unwrap();

      println!("  Merged into: {:?}", line);
    }

    merged_lines.push(line);
  }
  merged_lines
}




////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn join_similarly_oriented_interrupted_lines(
  mut lines: Vec<ConnectedLine>,
) -> Vec<ConnectedLine> {
  let mut merged_lines: Vec<ConnectedLine> = Vec::new();
  lines.sort();
  // lines.reverse();
  while let Some(mut line) = lines.pop() {
    while line.start_connects_to == Wall {
      println!("\nLooking for merges for {:?}...", line);
      if let Some(other) = lines.pop() {
        println!("  Considering {:?}...", other);
        if (line.start != other.end) || (other.end_connects_to != Wall) {
          println!("  Breaking!");
          break;
        }
        println!("  Could merge with {:?}.", other);
        line = ConnectedLine::new(
          line.orientation,
          other.start,
          line.end,
          other.start_connects_to,
          line.end_connects_to,
        )
        .unwrap();
        println!("  Merged into: {:?}", line);
      }
      else {
        break;
      }
    }
    println!("  Pushing {:?}.", line);
    merged_lines.push(line);
  }
  merged_lines
}













///////do not delete this comment
