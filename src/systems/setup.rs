use crate::resources::{
    dice::{Dice, Selected},
    game::GameState,
};
use bevy::{
    color::palettes::css::{BLACK, BLUE, GREY, WHITE},
    prelude::*,
};
use rand::Rng;

const POS: [(f32, f32); 6] = [
    // top row
    (-60.0, 0.0),
    (50.0, 0.0),
    (160.0, 0.0),
    // bot row
    (-60.0, -110.0),
    (50.0, -110.0),
    (160.0, -110.0),
];

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.init_resource::<Selected>();
    commands.init_resource::<Dice>();
    create_dice(&mut commands, server);
    create_buttons(&mut commands);
}

/// Roll and create starting dice
fn create_dice(commands: &mut Commands, server: Res<AssetServer>) {
    for i in 0..=5 {
        let p = POS[i];
        commands
            .spawn((
                Transform::from_xyz(p.0, p.1, 0.0),
                Sprite {
                    image: server.load(format!("d6_{}.png", rand::rng().random_range(1..=6))),
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
            ))
            .observe(dice_created)
            .observe(dice_none)
            .observe(dice_hover)
            .observe(dice_click);
    }
}

/// Create options buttons
fn create_buttons(commands: &mut Commands) {
    commands
        .spawn(Node {
            min_width: Val::Px(500.0),
            width: Val::Percent(30.0),
            max_width: Val::Px(1000.0),
            min_height: Val::Px(500.0),
            height: Val::Percent(100.0),
            max_height: Val::Px(1000.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            margin: UiRect {
                top: Val::Percent(1.0),
                bottom: Val::Percent(1.0),
                left: Val::Percent(0.0),
                right: Val::Percent(0.0),
            },
            ..default()
        })
        .with_children(|p| {
            for i in ["roll", "end"] {
                p.spawn((
                    Button,
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(10.0),
                        margin: UiRect::all(Val::Percent(1.0)),
                        border: UiRect::all(Val::Percent(1.5)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(BLACK.into()),
                    BorderRadius::MAX,
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ))
                .with_child((
                    Text::new(i),
                    TextFont {
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(WHITE.into()),
                ))
                .observe(button_none)
                .observe(button_release)
                .observe(button_click)
                .observe(button_hover);
            }
        });
}

fn dice_created(trigger: Trigger<OnAdd>, mut dice: ResMut<Dice>, query: Query<&Sprite>) {
    let e = trigger.entity();
    let sprite = query.get(e).expect("setup::dice_created query failed");
    let d = sprite
        .image
        .path()
        .expect("setup::dice_created bad asset path")
        .to_string()
        .split('_')
        .last()
        .expect("setup::dice_created bad asset name")
        .split('.')
        .next()
        .expect("setup::dice_created bad file extention")
        .trim()
        .parse::<u8>()
        .expect("setup::dice_created bad u8 input");
    let (f, i) = match dice.0.iter().position(|i| i.0 == e) {
        Some(i) => (true, i),
        None => (false, 0),
    };
    if f {
        // If entity is saved but dice has changed
        if dice.0[i].1 != d {
            dice.0[i] = (e, d);
            println!("{:?}", dice.0);
        }
    } else {
        // If entity is not saved, add it
        dice.0.push((e, d));
        println!("{:?}", dice.0);
    }
}

fn dice_none(trigger: Trigger<Pointer<Out>>, mut query: Query<&mut Sprite>) {
    let mut sprite = query.get_mut(trigger.entity()).unwrap();
    sprite.color = Color::srgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::new(100.0, 100.0));
}

fn dice_click(
    trigger: Trigger<Pointer<Click>>,
    mut selected: ResMut<Selected>,
    mut query: Query<&mut Sprite>,
) {
    let mut sprite = query.get_mut(trigger.entity()).unwrap();
    if !selected.0.contains(&trigger.entity()) {
        selected.0.push(trigger.entity());
        sprite.color = BLUE.into();
    } else {
        selected.0.retain(|i| i != &trigger.entity());
        sprite.color = GREY.into();
    }
}

fn dice_hover(trigger: Trigger<Pointer<Over>>, mut query: Query<&mut Sprite>) {
    let mut sprite = query.get_mut(trigger.entity()).unwrap();
    sprite.color = GREY.into();
    sprite.custom_size = Some(Vec2::new(105.0, 105.0));
}

fn button_none(trigger: Trigger<Pointer<Out>>, mut query: Query<&mut BorderColor, With<Button>>) {
    let mut border = query.get_mut(trigger.entity()).unwrap();
    border.0 = BLACK.into();
}

fn button_click(
    trigger: Trigger<Pointer<Down>>,
    mut new_state: ResMut<NextState<GameState>>,
    mut query: Query<(&Children, &mut BorderColor), With<Button>>,
    text: Query<&Text>,
) {
    let (c, mut b) = query.get_mut(trigger.entity()).unwrap();
    b.0 = BLUE.into();
    let t = text.get(c[0]).unwrap();
    if t.0 == "roll" {
        new_state.set(GameState::Roll);
    }
}

fn button_release(trigger: Trigger<Pointer<Up>>, mut query: Query<&mut BorderColor, With<Button>>) {
    let mut border = query.get_mut(trigger.entity()).unwrap();
    border.0 = WHITE.into();
}

fn button_hover(trigger: Trigger<Pointer<Over>>, mut query: Query<&mut BorderColor, With<Button>>) {
    let mut border = query.get_mut(trigger.entity()).unwrap();
    border.0 = WHITE.into();
}
