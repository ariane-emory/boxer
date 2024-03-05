#![allow(dead_code)]

mod connected_line;
mod find_rectangles;
mod flippable;
mod line;
mod line_methods;
mod offsetable;
mod orientation;
mod point;
mod positional;
mod rectangle;
mod size;
mod word;

use crate::util::*;

pub use connected_line::*;
pub use find_rectangles::*;
pub use flippable::*;
pub use line::*;
pub use line_methods::*;
pub use offsetable::*;
pub use orientation::*;
pub use point::*;
pub use positional::*;
pub use rectangle::*;
pub use size::*;
pub use word::*;

pub type GeoResult<T> = std::result::Result<T, ErrString>;
