use crate::prelude::*;

mod player_input;
mod sync_grid;
mod sync_cam;
mod collision;
mod random_move;

pub struct CoreSystems;

impl Plugin for CoreSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_input::system);
        app.add_systems(Update, collision::system);
        app.add_systems(Update, sync_grid::system);
        app.add_systems(Update, sync_cam::system);
        app.add_systems(Update, random_move::system);
    }
}