use crate::prelude::*;

pub fn end_turn(current_turn_state: Res<State<TurnState>>,
                mut next_turn_state: ResMut<NextState<TurnState>>,
                mut next_play_state: ResMut<NextState<PlayState>>,
                player: Query<(&Health, &Position), With<Player>>,
                amulet: Query<&Position, With<AmuletOfYala>>,
) {
    if let Some((player_health, player_position)) = player.get_single().ok() {
        if let Some(amulet) = amulet.get_single().ok() {
            if amulet == player_position {
                next_play_state.set(PlayState::Victory)
            }
        }

        if player_health.current < 1 {
            println!("dead");
            next_play_state.set(PlayState::Defeat)
        }
    }
    println!("{:?} => {:?}", current_turn_state.get(), current_turn_state.next() );
    next_turn_state.set(current_turn_state.next());
}