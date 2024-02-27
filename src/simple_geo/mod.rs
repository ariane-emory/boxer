#![allow(dead_code)]

pub mod find_rectangles;
pub mod line;
pub mod line_methods;
pub mod orientation;
pub mod point;
pub mod positional;
pub mod rectangle;
pub mod size;

use crate::util::*;

pub use find_rectangles::*;
pub use line::*;
pub use line_methods::*;
pub use orientation::*;
pub use point::*;
pub use positional::*;
pub use rectangle::*;
pub use size::*;

pub type GeoResult<T> = std::result::Result<T, ErrString>;
