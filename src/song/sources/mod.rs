
mod base;
mod smart;
mod windowed;
mod stoppable;

use super::*;

pub use self::base::BaseSource;
pub use self::smart::SmartSource;
pub use self::windowed::FloatWindowSource;
pub use self::stoppable::StoppableSource;

pub type Sample = i16;
