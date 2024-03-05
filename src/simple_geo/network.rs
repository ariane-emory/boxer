use crate::simple_geo::ConnectedLine;
use crate::simple_geo::ConnectionType;
use crate::simple_geo::Point;
use std::collections::{HashMap, HashSet};

////////////////////////////////////////////////////////////////////////////////
pub fn find_networks(lines: &Vec<ConnectedLine>) -> Vec<Vec<ConnectedLine>> {
  let mut graph: HashMap<Point, Vec<ConnectedLine>> = HashMap::new();

  for line in lines {
    graph.entry(line.start).or_default().push(*line);
    graph.entry(line.end).or_default().push(*line);
  }

  let mut visited_points: HashSet<Point> = HashSet::new();
  let mut visited_lines: HashSet<ConnectedLine> = HashSet::new();
  let mut networks: Vec<Vec<ConnectedLine>> = Vec::new();

  fn dfs(
    point: &Point,
    graph: &HashMap<Point, Vec<ConnectedLine>>,
    visited_points: &mut HashSet<Point>,
    visited_lines: &mut HashSet<ConnectedLine>,
    network: &mut Vec<ConnectedLine>,
  ) {
    if visited_points.insert(*point) {
      if let Some(lines) = graph.get(point) {
        for line in lines {
          if visited_lines.insert(*line) {
            network.push(*line);
            let next_point = if line.start == *point {
              &line.end
            }
            else {
              &line.start
            };
            dfs(next_point, graph, visited_points, visited_lines, network);
          }
        }
      }
    }
  }

  // Find and group networks
  for point in graph.keys() {
    if !visited_points.contains(point) {
      let mut network = Vec::new();
      dfs(point, &graph, &mut visited_points, &mut visited_lines, &mut network);
      if !network.is_empty() {
        networks.push(network);
      }
    }
  }

  networks
}

////////////////////////////////////////////////////////////////////////////////
pub fn network_get_endpoints(
  network: &Vec<ConnectedLine>,
) -> Vec<(Point, ConnectionType)> {
  let mut point_connection_map: HashMap<Point, Vec<ConnectionType>> =
    HashMap::new();

  // Populate the map with connection types for each point
  for line in network {
    point_connection_map
      .entry(line.start)
      .or_insert_with(Vec::new)
      .push(line.start_connects_to);
    point_connection_map
      .entry(line.end.clone())
      .or_insert_with(Vec::new)
      .push(line.end_connects_to);
  }

  // Filter points that are connected exactly once, and pair them with their
  // connection type
  let end_points = point_connection_map
    .into_iter()
    .filter_map(|(point, connections)| {
      if connections.len() == 1 {
        // Assume there's only one connection type if the point is unique
        Some((point, connections[0]))
      }
      else {
        None
      }
    })
    .collect::<Vec<_>>();

  end_points
}
