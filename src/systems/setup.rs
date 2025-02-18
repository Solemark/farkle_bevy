use crate::resources::{
    dice::{create_dice_rows, SelectedDice},
    game::{create_option_row, Status, StatusUI},
};
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
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
