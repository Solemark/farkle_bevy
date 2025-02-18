use crate::resources::game::{Status, StatusUI};
use bevy::prelude::*;

pub fn status_system(status: Res<Status>, mut query: Query<&mut Text, With<StatusUI>>) {
    let mut text = query.single_mut();
    text.0 = status.0.to_string();
}
