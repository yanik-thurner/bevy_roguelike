use bevy::ecs::schedule::{LogLevel, ScheduleBuildSettings};

use prelude::*;

use crate::core::Core;

mod core;

mod prelude {
    pub use bevy::prelude::*;

    pub use crate::core::prelude::*;
}

const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 50;

fn main() {
    App::new()
        // Enable ambiguity warnings for the Update schedule
        .edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(Core)
        .insert_resource(Msaa::Off)
        .run();
}
