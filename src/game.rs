use bevy::{
    color::Color,
    ecs::{bundle::Bundle, component::Component},
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder},
    prelude::{Resource, States},
    text::{TextColor, TextFont},
    ui::{
        widget::{Button, Text},
        AlignItems, BackgroundColor, BorderColor, BorderRadius, JustifyContent, Node, UiRect, Val,
    },
    utils::default,
};

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    Check,
    EndTurn,
    Farkle,
    #[default]
    Play,
    Roll,
}

#[derive(Component)]
pub struct StatusUI;
#[derive(Resource)]
pub struct Status(pub String);
impl Default for Status {
    fn default() -> Self {
        Self("player's roll".to_string())
    }
}

#[derive(Component)]
pub struct OptionUI;
#[derive(Bundle)]
struct OptionButton {
    button: Button,
    node: Node,
}
impl OptionButton {
    fn new() -> Self {
        Self {
            button: Button,
            node: Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        }
    }
}

pub fn create_option_row(p: &mut ChildBuilder) {
    p.spawn(Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    })
    .with_children(create_option_buttons);
}

pub const PLAY: &'static str = "play";
pub const ROLL: &'static str = "roll";
pub const END: &'static str = "end";
fn create_option_buttons(p: &mut ChildBuilder) {
    for i in [PLAY, ROLL, END] {
        p.spawn((
            OptionUI,
            OptionButton::new(),
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        ))
        .with_child((
            Text::new(i),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
    }
}
