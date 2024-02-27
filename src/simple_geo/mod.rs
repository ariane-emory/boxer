#![allow(dead_code)]

use crate::util::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub type GeoResult<T> = std::result::Result<T, ErrString>;
////////////////////////////////////////////////////////////////////////////////////////////////////
mod free_functions;
mod line;
mod orientation;
mod point;
mod positional;
mod rectangle;
mod size;

pub use free_functions::*;
pub use line::*;
pub use orientation::*;
pub use point::*;
pub use positional::*;
pub use rectangle::*;
pub use size::*;
