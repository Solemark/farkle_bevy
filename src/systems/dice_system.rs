use crate::resources::dice::Selected;
use bevy::{color::palettes::css::BLUE, prelude::*};

pub fn selected_dice_system(selected: Res<Selected>, mut query: Query<(Entity, &mut Sprite)>) {
    for (e, mut s) in &mut query {
        if selected.0.contains(&e) {
            s.color = BLUE.into();
            s.custom_size = Some(Vec2::new(105.0, 105.0));
        }
    }
}
