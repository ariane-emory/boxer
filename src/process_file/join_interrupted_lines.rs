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
  let mut vertical_lines = lines
    .iter()
    .filter(|cl| cl.orientation == Vertical)
    .cloned()
    .collect::<Vec<ConnectedLine>>();

  vertical_lines.sort_by_key(|k| k.start.col);

  vertical_lines
    .iter()
    .for_each(|line| println!("Vert Line:          {:?}", line));
  horizontal_lines
    .iter()
    .for_each(|line| println!("Horiz Line:         {:?}", line));

  println!("\n--------------------------------------------------------------------------------");
  let vertical_lines =
    join_similarly_oriented_interrupted_lines(vertical_lines);
  println!("\n--------------------------------------------------------------------------------");
  let horizontal_lines =
    join_similarly_oriented_interrupted_lines(horizontal_lines);

  let mut lines = Vec::new();

  lines.extend(horizontal_lines);
  lines.extend(vertical_lines);
  lines.sort();
  lines
}

////////////////////////////////////////////////////////////////////////////////
pub fn join_similarly_oriented_interrupted_lines(
  mut lines: Vec<ConnectedLine>,
) -> Vec<ConnectedLine> {
  let mut merged_lines: Vec<ConnectedLine> = Vec::new();
  lines.sort();
  lines.reverse();

  while let Some(mut line) = lines.pop() {
    println!("Looking for merges for {:?}...", line);

    if line.end_connects_to == Wall {
      while let Some(other) = lines.remove_if(|o| {
        line.orientation == o.orientation
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
    }
    merged_lines.push(line);
  }
  merged_lines
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn buggy_join_similarly_oriented_interrupted_lines(
  mut lines: Vec<ConnectedLine>,
) -> Vec<ConnectedLine> {
  let mut merged_lines: Vec<ConnectedLine> = Vec::new();

  while let Some(mut line) = lines.pop() {
    println!("\nLooking for merges for       {:?}...", line);

    while line.start_connects_to == Wall {
      if let Some(other) = lines.pop() {
        println!("  Considering                {:?}...", other);
        if (line.start != other.end) || (other.end_connects_to != Wall) {
          println!("  Breaking!");
          break;
        }
        //println!("  Will merge with            {:?}.", other);
        line = ConnectedLine::new(
          line.orientation,
          other.start,
          line.end,
          other.start_connects_to,
          line.end_connects_to,
        )
        .unwrap();
        println!("  Merged into:               {:?}", line);
      }
      else {
        break;
      }
    }
    println!("  Pushing                    {:?}.", line);
    merged_lines.push(line);
  }
  merged_lines
}
