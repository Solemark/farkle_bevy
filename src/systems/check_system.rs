use crate::resources::{
    dice::{Dice, Selected},
    game::GameState,
};
use bevy::prelude::*;

pub fn check_system(
    dice: Res<Dice>,
    selected: Res<Selected>,
    mut state: ResMut<NextState<GameState>>,
) {
    let mut d = Vec::new();
    for (e, i) in dice.0.clone() {
        if !selected.0.contains(&e) {
            d.push(i);
        }
    }
    d.sort();
    if !scoring(&d) {
        state.set(GameState::Farkle);
    } else {
        state.set(GameState::Play);
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
