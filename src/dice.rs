use bevy::{
    color::Color,
    ecs::bundle::Bundle,
    prelude::{BuildChildren, ChildBuild, ChildBuilder, Text},
    text::{TextColor, TextFont},
    ui::{
        widget::Button, AlignItems, BackgroundColor, BorderColor, BorderRadius, JustifyContent,
        Node, UiRect, Val,
    },
    utils::default,
};

#[derive(Bundle)]
struct DiceButton {
    button: Button,
    node: Node,
}
impl DiceButton {
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

pub fn create_buttons(p: &mut ChildBuilder) {
    for _ in 0..2 {
        p.spawn(Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(create_button_row);
    }
}

fn create_button_row(p: &mut ChildBuilder) {
    for _ in 0..=2 {
        p.spawn((
            DiceButton::new(),
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        ))
        .with_child((
            Text::new("0"),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
    }
}
