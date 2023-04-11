//! This crate is about everything concerning the highest-level, application layer of a Sinan app.

mod app;

pub use app::*;

#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        app::App,
    };
}
