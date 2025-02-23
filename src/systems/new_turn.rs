use crate::resources::{
    dice::SelectedDice,
    game::{GameState, Status},
};
use bevy::prelude::*;

pub fn new_turn(
    mut status: ResMut<Status>,
    mut selected_dice: ResMut<SelectedDice>,
    mut new_state: ResMut<NextState<GameState>>,
) {
    status.0 = "new player's turn".to_string();
    selected_dice.0 = Vec::new();
    new_state.set(GameState::Roll);
}
