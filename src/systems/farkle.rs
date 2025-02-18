use crate::resources::{
    dice::SelectedDice,
    game::{GameState, Status, END},
};
use bevy::color::palettes::css::{BLACK, BLUE, RED, WHITE};
use bevy::prelude::*;

use super::types::OBQuery;

pub fn farkle_system(
    mut status: ResMut<Status>,
    mut selected_dice: ResMut<SelectedDice>,
    mut new_state: ResMut<NextState<GameState>>,
    mut button_query: Query<(&Interaction, &mut BorderColor, &Children), OBQuery>,
    text_query: Query<&mut Text>,
) {
    // RESET PLAYER SCORE
    status.0 = "farkle".to_string();
    selected_dice.0 = Vec::new();

    for (i, mut b, c) in &mut button_query {
        let text = text_query.get(c[0]).unwrap();
        match i {
            Interaction::Pressed => {
                if text.0 == END {
                    new_state.set(GameState::EndTurn)
                }
            }
            Interaction::Hovered => {
                if text.0 == END {
                    b.0 = WHITE.into()
                } else {
                    b.0 = RED.into()
                }
            }
            Interaction::None => {
                if text.0 == END {
                    b.0 = BLUE.into()
                } else {
                    b.0 = BLACK.into()
                }
            }
        }
    }
}
