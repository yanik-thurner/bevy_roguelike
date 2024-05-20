use bevy::ecs::schedule::{LogLevel, ScheduleBuildSettings};

use prelude::*;
use crate::core::Core;

mod components;
mod core_systems;
mod core_logic;
mod states;
mod system_sets;
mod events;
mod game_logic;
mod core;

mod prelude {
    pub use bevy::prelude::*;

    pub use crate::components::*;
    pub use crate::core::prelude::*;
    pub use crate::core_logic::*;
    pub use crate::core_systems::*;
    pub use crate::events::*;
    pub use crate::game_logic::prelude::*;
    pub use crate::states::*;

    pub const SPRITE_SIZE: f32 = 32.0;
    pub const SPRITE_SHEET_WALL: usize = 35;
    pub const SPRITE_SHEET_FLOOR: usize = 46;
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
        .add_plugins(CoreSystems)
        .insert_resource(Msaa::Off)
        .run();
}
