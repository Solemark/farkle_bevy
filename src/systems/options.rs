use crate::resources::game::{GameState, Status, END, ROLL};
use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use super::types::OBQuery;

pub fn option_buttons(
    mut new_state: ResMut<NextState<GameState>>,
    mut status: ResMut<Status>,
    mut button_query: Query<(&Interaction, &mut BorderColor, &Children), OBQuery>,
    text_query: Query<&mut Text>,
) {
    for (i, mut b, c) in &mut button_query {
        let text = text_query.get(c[0]).unwrap();
        match i {
            Interaction::Pressed => {
                if text.0 == ROLL {
                    status.0 = "rolling".to_string();
                    new_state.set(GameState::Roll);
                }
                if text.0 == END {
                    status.0 = "end turn".to_string();
                    new_state.set(GameState::EndTurn);
                }
            }
            Interaction::Hovered => b.0 = WHITE.into(),
            Interaction::None => b.0 = BLACK.into(),
        }
    }
}
