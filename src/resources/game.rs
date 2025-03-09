use bevy::prelude::*;

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
pub struct ScoreUI;
#[derive(Resource)]
pub struct Score(pub usize);
impl Default for Score {
    fn default() -> Self {
        Self(0)
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

pub const ROLL: &str = "roll";
pub const END: &str = "end";
fn create_option_buttons(p: &mut ChildBuilder) {
    for i in [ROLL, END] {
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
