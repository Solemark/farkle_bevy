use bevy::prelude::{Resource, States};

#[derive(Resource)]
pub struct TurnCounter(pub u8);
impl Default for TurnCounter {
    fn default() -> Self {
        Self(0)
    }
}
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    New,
    Play,
    Roll,
    Remove,
    Wait,
}
