use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Resource)]
pub struct Selected(pub Vec<Entity>);
impl Default for Selected {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Resource)]
pub struct Dice(pub Vec<(Entity, u8)>);
impl Default for Dice {
    fn default() -> Self {
        Self(Vec::new())
    }
}
