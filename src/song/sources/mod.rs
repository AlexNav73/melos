
mod base;
mod smart;
mod windowed;

use super::*;

pub use self::base::BaseSource;
pub use self::smart::SmartSource;
pub use self::windowed::FloatWindowSource;

pub type Sample = i16;
