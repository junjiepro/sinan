use bevy_app::App as BevyApp;

pub struct App {
    pub bevy_app: BevyApp,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self { bevy_app: Default::default() };
        app.bevy_app.add_plugin(sinan_plugin::PluginManage);

        app
    }
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
