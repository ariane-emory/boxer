use crate::simple_geo::network_get_endpoints;
use crate::simple_geo::ConnectedLine;
use crate::simple_geo::ConnectionType::*;
use crate::simple_geo::Rectangle;

////////////////////////////////////////////////////////////////////////////////
pub fn check_for_illegal_networks(
  networks: &Vec<Vec<ConnectedLine>>,
  rectangles: &Vec<Rectangle>,
) {
  for (i, network) in networks.iter().enumerate() {
    let endpoints = network_get_endpoints(&network);

    for (ii, (point, point_type)) in endpoints.iter().enumerate() {
      // A network is illegal if any of it's end points are on a Rectangle's
      // corner:
      if *point_type == Corner {
        for (iii, rectangle) in rectangles.iter().enumerate() {
          if rectangle.has_corner_point(*point) {
            panic!(
            "Network #{} has an illegal end point #{}: {:?} on Rectangle #{}'s corner.",
            i + 1,
            ii + 1,
            (point, point_type),
            iii + 1
          );
          }
        }
      }

      // A network is illegal if any of it's end points are on a AnotherLine
      // that is not part of any Rectangle:
      if *point_type == AnotherLine {
        let mut found_it = false;
        for (_iii, rectangle) in rectangles.iter().enumerate() {
          if rectangle.has_wall_point(*point) {
            // println!(
            //   "Found wall point on Rectangle #{}'s wall for end Point #{} in
            // network #{}.",   iii + 1,
            //   ii + 1,
            //   i + 1
            // );
            found_it = true;
            break;
          }
        }
        if !found_it {
          panic!(
          "Network #{} has an illegal end point #{}: {:?} on a AnotherLine that is not part of any Rectangle.",
          i + 1,
          ii + 1,
          (point, point_type)
        );
        }
      }
    }
  }
}
