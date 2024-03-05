use crate::simple_geo::*;
use std::collections::{HashMap, HashSet};
use std::fmt;
use ConnectionType::*;

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum ConnectionType {
  Nothing,
  Corner,
  Wall,
}
////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct ConnectedLine {
  pub orientation: Orientation,
  pub start: Point,
  pub end: Point,
  pub start_connects_to: ConnectionType,
  pub end_connects_to: ConnectionType,
}
////////////////////////////////////////////////////////////////////////////////
impl ConnectedLine {
  pub fn new(
    orientation: Orientation,
    start: Point,
    end: Point,
    start_connects_to: ConnectionType,
    end_connects_to: ConnectionType,
  ) -> GeoResult<Self> {
    let (start, end) = Line::new(start, end)?.points();

    Ok(Self {
      start,
      end,
      orientation,
      start_connects_to,
      end_connects_to,
    })
  }

  pub fn is_corner_connected(&self) -> bool {
    self.start_connects_to == Corner && self.end_connects_to == Corner
  }

  ////////////////////////////////////////////////////////////////////////////////
  pub fn find_chains(lines: &Vec<ConnectedLine>) -> Vec<Vec<ConnectedLine>> {
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
  pub fn analyze_chain(
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
}
////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for ConnectedLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let orientation_str = match self.orientation {
      Orientation::Horizontal => "H",
      Orientation::Vertical => "V",
    };
    let connection_str = format!(
      "{:?}←{}→{:?}",
      self.start_connects_to,
      self.len(),
      self.end_connects_to
    );
    write!(
      f,
      "{}{:?} {:15} {:?}",
      orientation_str, self.start, connection_str, self.end
    )
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Positional for ConnectedLine {
  fn top_left(&self) -> Point {
    self.start
  }

  fn bottom_right(&self) -> Point {
    self.end
  }
}
////////////////////////////////////////////////////////////////////////////////
impl LineMethods for ConnectedLine {}
////////////////////////////////////////////////////////////////////////////////
impl Offsetable for ConnectedLine {
  fn offset_by(&self, line_offset: isize, col_offset: isize) -> Self {
    Self::new(
      self.orientation,
      self.start.offset_by(line_offset, col_offset),
      self.end.offset_by(line_offset, col_offset),
      self.start_connects_to,
      self.end_connects_to,
    )
    .unwrap()
  }
}
////////////////////////////////////////////////////////////////////////////////
impl Flippable for ConnectedLine {
  fn flip(&self) -> Self {
    Self::new(
      if self.orientation == Orientation::Horizontal {
        Orientation::Vertical
      }
      else {
        Orientation::Horizontal
      },
      self.start().flip(),
      self.end().flip(),
      self.start_connects_to,
      self.end_connects_to,
    )
    .unwrap()
  }
}
