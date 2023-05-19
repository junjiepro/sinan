//! The sinan plugin lifecycle manage.

#[cfg(feature = "manage")]
use bevy_app::Plugin;
#[cfg(feature = "manage")]
use bevy_ecs::{system::{Query, Commands}, prelude::Entity, query::With};
use bevy_ecs::component::Component;
use semver::Version;

/// Plugin metatdata
#[derive(Component)]
pub struct PluginMeta {
    id: String,
    version: Version,
    source: String,
}

impl PluginMeta {
    pub fn is_updatable(&self, plugin_meta: &PluginMeta) -> bool {
        self.id == plugin_meta.id && self.version < plugin_meta.version
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }
}

#[derive(Component)]
pub struct Installed;

#[derive(Component)]
pub struct Installable;

#[derive(Component)]
pub struct Uninstall;

#[cfg(feature = "manage")]
fn uninstall_system(
    mut commands: Commands,
    entities: Query<Entity, (With<PluginMeta>, With<Installed>, With<Uninstall>)>,
) {
    for entity in entities.iter() {
        commands
            .entity(entity)
            .remove::<Installed>()
            .remove::<Uninstall>();
    }
}

#[cfg(feature = "manage")]
pub struct  PluginManage;

#[cfg(feature = "manage")]
impl Plugin for PluginManage {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_system(uninstall_system);
    }
}