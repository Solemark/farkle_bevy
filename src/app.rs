use bevy::{
    app::{App, Startup, Update},
    color::palettes::css::{BLACK, WHITE},
    prelude::{
        in_state, AppExtStates, BuildChildren, Button, Camera2d, Changed, Children, Commands,
        Entity, IntoSystemConfigs, NextState, Query, Res, ResMut, State, States, Text, With,
    },
    ui::{AlignItems, BorderColor, FlexDirection, Interaction, JustifyContent, Node, Val},
    utils::default,
    winit::WinitSettings,
    DefaultPlugins,
};
use rand::Rng;

use crate::{
    dice::create_buttons,
    turn_counter::{GameState, TurnCounter},
};

pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .insert_state(GameState::New)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                new_game.run_if(in_state(GameState::New)),
                roll_dice.run_if(in_state(GameState::Roll)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.init_resource::<TurnCounter>();
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(create_buttons);
}

pub fn new_game(mut new_state: ResMut<NextState<GameState>>) {
    new_state.set(GameState::Roll);
}

pub fn roll_dice(
    mut new_state: ResMut<NextState<GameState>>,
    mut button_query: Query<&Children, With<Button>>,
    mut text_query: Query<&mut Text>,
) {
    for c in &mut button_query {
        let mut text = text_query.get_mut(c[0]).unwrap();
        text.0 = rand::rng().random_range(1..=6).to_string();
    }
    new_state.set(GameState::Play);
}
