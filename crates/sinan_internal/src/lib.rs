//! This module is separated into its own crate to enable simple dynamic linking for Sinan, and should not be used directly

/// `use sinan::prelude::*;` to import common components, bundles, and plugins.
pub mod prelude;

pub mod app {
    //! Build sinan apps, create plugins, and read events.
    pub use sinan_app::*;
}

pub mod plugin {
    //! Build sinan apps, create plugins, and read events.
    pub use sinan_plugin::*;
}
