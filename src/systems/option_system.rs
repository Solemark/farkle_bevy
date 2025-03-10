use crate::resources::{dice::Selected, game::GameState};
use bevy::prelude::*;
use rand::Rng;

pub fn roll_system(
    server: Res<AssetServer>,
    selected: Res<Selected>,
    mut new_state: ResMut<NextState<GameState>>,
    mut query: Query<(Entity, &mut Sprite)>,
    // dice: ResMut<Dice>,
) {
    for (e, mut s) in &mut query {
        if !selected.0.contains(&e) {
            s.image = server.load(format!("d6_{}.png", rand::rng().random_range(1..=6)));
        }
    }
    new_state.set(GameState::Play);
}
