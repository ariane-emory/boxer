#![allow(dead_code)]

mod free_functions;
mod line;
mod orientation;
mod point;
mod positional;
mod rectangle;
mod size;

use crate::util::*;
pub use free_functions::*;
pub use line::*;
pub use orientation::*;
pub use point::*;
pub use positional::*;
pub use rectangle::*;
pub use size::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub type GeoResult<T> = std::result::Result<T, ErrString>;
