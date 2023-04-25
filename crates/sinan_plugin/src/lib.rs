//! This crate is about plugin lifecycle manage.

mod plugin;

pub use plugin::*;

#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        plugin::*,
    };
}
