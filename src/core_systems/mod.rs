mod player_input;
mod sync_grid;
mod sync_cam;

use crate::prelude::*;

pub struct CoreSystems;

impl Plugin for CoreSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_input::system);
        app.add_systems(Update, sync_grid::system);
        app.add_systems(Update, sync_cam::system);
    }
}