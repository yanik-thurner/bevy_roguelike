mod components;
mod map;
mod map_builder;
mod spawner;
mod core_systems;

mod prelude {
    pub const SPRITE_SIZE: i32 = 32;
    pub const SPRITE_SHEET_WALL: usize = 35;
    pub const SPRITE_SHEET_FLOOR: usize = 46;

    pub use bevy::prelude::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::spawner::*;
    pub use crate::core_systems::*;
}

use bevy::render::view::RenderLayers;
use rand::rngs::ThreadRng;
use prelude::*;
use crate::map_builder::MapBuilder;

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.scale /= 2.0;

    commands.spawn((PlayerCamera, camera, RenderLayers::from_layers(&[0, 1, 2])));

}

fn bevy_main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (map_builder::system, setup, spawn_player, spawn_random_monsters).chain())
        .add_plugins(CoreSystems)
        .insert_resource(Msaa::Off)
        .run();
}

fn main() {
    //breakout::main();
    bevy_main();
}
