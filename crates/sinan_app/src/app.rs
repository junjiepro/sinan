use bevy_app::App as BevyApp;

#[derive(Default)]
pub struct App {
    pub bevy_app: BevyApp,
}

impl App {
    /// Creates a new [`App`] with some default structure to enable core engine features.
    /// This is the preferred constructor for most use cases.
    ///
    /// This calls [`App::add_default_schedules`].
    pub fn new() -> App {
        App::default()
    }
}
