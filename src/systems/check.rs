use super::types::DBQuery;
use crate::resources::{dice::SelectedDice, game::GameState};
use bevy::prelude::*;

pub fn check_system(
    selected_dice: Res<SelectedDice>,
    mut new_state: ResMut<NextState<GameState>>,
    mut button_query: Query<(Entity, &Children), DBQuery>,
    text_query: Query<&mut Text>,
) {
    let mut dice = Vec::new();
    for (e, c) in &mut button_query {
        let text = text_query.get(c[0]).unwrap();
        if !selected_dice.0.contains(&e) {
            dice.push(text.0.trim().parse::<u8>().unwrap_or(0));
        }
    }
    if !scoring(&dice) {
        new_state.set(GameState::Farkle);
    } else {
        new_state.set(GameState::Play);
    }
}

fn scoring(dice: &[u8]) -> bool {
    if dice.contains(&1) {
        return true;
    }
    if dice.contains(&5) {
        return true;
    }
    if dice.len() >= 3 && dice[0] == dice[1] && dice[0] == dice[2] {
        return true;
    }
    false
}
