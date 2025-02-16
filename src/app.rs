use bevy::{
    color::palettes::css::{BLACK, BLUE, RED, WHITE},
    prelude::*,
    winit::WinitSettings,
};
use rand::Rng;

use crate::{
    dice::{create_dice_rows, SelectedDice},
    game::{create_option_row, GameState, OptionUI, Status, StatusUI, END, ROLL},
};

/// Option button query params
type OBQuery = (With<Button>, With<OptionUI>);
/// Dice button query params
type DBQuery = (With<Button>, Without<OptionUI>);

pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .insert_state(GameState::Roll)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                status_system,
                dice_buttons,
                roll.run_if(in_state(GameState::Roll)),
                option_buttons.run_if(in_state(GameState::Play)),
                check_system.run_if(in_state(GameState::Check)),
                farkle_system.run_if(in_state(GameState::Farkle)),
                new_player_turn.run_if(in_state(GameState::EndTurn)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.init_resource::<Status>();
    commands.init_resource::<SelectedDice>();
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(create_dice_rows)
        .with_children(create_option_row)
        .with_child((
            StatusUI,
            Text::new("status message"),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
}

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
        new_state.set(GameState::Farkle)
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

pub fn status_system(status: Res<Status>, mut query: Query<&mut Text, With<StatusUI>>) {
    let mut text = query.single_mut();
    text.0 = status.0.to_string();
}

pub fn new_player_turn(
    mut status: ResMut<Status>,
    mut selected_dice: ResMut<SelectedDice>,
    mut new_state: ResMut<NextState<GameState>>,
) {
    status.0 = "new player's turn".to_string();
    selected_dice.0 = Vec::new();
    new_state.set(GameState::Roll);
}
