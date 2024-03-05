use crate::simple_geo::ConnectedLine;
use crate::simple_geo::Point;
use std::collections::{HashMap, HashSet};

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
