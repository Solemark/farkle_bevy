use super::types::DBQuery;
use crate::resources::{dice::SelectedDice, game::GameState};
use bevy::color::palettes::css::{BLACK, BLUE, RED, WHITE};
use bevy::prelude::*;

pub fn dice_buttons(
    state: Res<State<GameState>>,
    mut selected_dice: ResMut<SelectedDice>,
    mut button_query: Query<(&Interaction, &mut BorderColor, Entity), DBQuery>,
) {
    if state.get() == &GameState::Farkle {
        for (_, mut b, _) in &mut button_query {
            b.0 = RED.into();
        }
    } else {
        for (i, mut b, e) in &mut button_query {
            match i {
                Interaction::Pressed => {
                    if !selected_dice.0.contains(&e) {
                        b.0 = BLUE.into();
                        selected_dice.0.push(e);
                    } else {
                        b.0 = RED.into();
                        let i = selected_dice.0.iter().position(|i| i == &e).unwrap();
                        selected_dice.0.remove(i);
                    }
                }
                Interaction::Hovered => {
                    if selected_dice.0.contains(&e) {
                        b.0 = RED.into();
                    } else {
                        b.0 = WHITE.into()
                    }
                }
                Interaction::None => {
                    if selected_dice.0.contains(&e) {
                        b.0 = BLUE.into();
                    } else {
                        b.0 = BLACK.into()
                    }
                }
            }
        }
    }
}
