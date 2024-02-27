#![allow(dead_code)]

mod find_rectangles;
mod line;
mod orientation;
mod point;
mod positional;
mod rectangle;
mod size;

use crate::util::*;

pub use find_rectangles::*;
pub use line::*;
pub use orientation::*;
pub use point::*;
pub use positional::*;
pub use rectangle::*;
pub use size::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub type GeoResult<T> = std::result::Result<T, ErrString>;
