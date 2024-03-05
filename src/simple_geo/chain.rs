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
pub fn chain_get_network(chain: &[ConnectedLine]) -> Vec<Point> {
  let mut connectivity_map = HashMap::new();

  for line in chain {
    *connectivity_map.entry(line.start.clone()).or_insert(0) += 1;
    *connectivity_map.entry(line.end.clone()).or_insert(0) += 1;
  }

  let end_points = connectivity_map
    .into_iter()
    .filter(|&(_, count)| count == 1)
    .map(|(point, _)| point)
    .collect::<Vec<_>>();

  end_points
}
