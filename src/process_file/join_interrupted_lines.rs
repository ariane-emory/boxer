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
  //horizontal_lines.sort_by_key(|k| k.start.line);
  //horizontal_lines.reverse();

  let mut vertical_lines = lines
    .iter()
    .filter(|cl| cl.orientation == Vertical)
    .cloned()
    .collect::<Vec<ConnectedLine>>();
  vertical_lines.sort_by_key(|k| k.start.col);
  //vertical_lines.reverse();

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
#[allow(dead_code)]
fn join_similarly_oriented_interrupted_lines(
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













///////do not delete this comment
