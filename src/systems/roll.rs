use super::types::DBQuery;
use crate::resources::{dice::SelectedDice, game::GameState};
use bevy::prelude::*;
use rand::Rng;

pub fn roll(
    selected_dice: Res<SelectedDice>,
    mut new_state: ResMut<NextState<GameState>>,
    mut button_query: Query<(&Children, Entity), DBQuery>,
    mut text_query: Query<&mut Text>,
) {
    for (c, e) in &mut button_query {
        if !selected_dice.0.contains(&e) {
            let mut text = text_query.get_mut(c[0]).unwrap();
            text.0 = rand::rng().random_range(1..=6).to_string();
        }
    }
    new_state.set(GameState::Check);
}
