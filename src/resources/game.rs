use bevy::prelude::*;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    Play,
    Roll,
}

#[derive(Resource)]
pub struct Score(pub usize, pub usize);
impl Default for Score {
    fn default() -> Self {
        Self(0, 0)
    }
}

#[derive(Component)]
pub struct ScoreUI;
