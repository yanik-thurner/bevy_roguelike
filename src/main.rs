use bevy::ecs::schedule::{LogLevel, ScheduleBuildSettings};

use prelude::*;

mod components;
mod core_systems;
mod core_logic;
mod states;
mod system_sets;
mod events;

mod prelude {
    pub use bevy::prelude::*;

    pub use crate::components::*;
    pub use crate::core_logic::*;
    pub use crate::core_systems::*;
    pub use crate::events::*;
    pub use crate::states::*;

    pub const SPRITE_SIZE: f32 = 32.0;
    pub const SPRITE_SHEET_WALL: usize = 35;
    pub const SPRITE_SHEET_FLOOR: usize = 46;
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.scale /= 1.0;

    commands.spawn((PlayerCamera, camera));
}

fn bevy_main() {
    App::new()
        // Enable ambiguity warnings for the Update schedule
        .edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup, map_builder::system, spawner::spawn_player, spawner::spawn_random_monsters).chain())
        .add_plugins(CoreSystems)
        .insert_resource(Msaa::Off)
        .run();
}

fn main() {
    //breakout::main();
    bevy_main();
}
