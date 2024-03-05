use crate::simple_geo::ConnectedLine;
use crate::simple_geo::ConnectionType::*;
use crate::util::RemoveIf;

////////////////////////////////////////////////////////////////////////////////
pub fn merge_interrupted_lines(
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

    merged_lines.push(line);
  }
  merged_lines
}
