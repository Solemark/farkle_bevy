use crate::resources::dice::Selected;
use bevy::prelude::*;

pub fn selected_dice_system(
    selected: Res<Selected>,
    mut query: Query<(Entity, &mut Sprite, &mut Transform)>,
) {
    for (e, mut s, mut t) in &mut query {
        if selected.0.contains(&e) {
            s.custom_size = Some(Vec2::new(105.0, 105.0));
            t.translation.y = -150.0;
        } else {
            t.translation.y = -200.0;
        }
    }
}
