mod resources;
mod systems;

use crate::systems::setup::setup;
use bevy::{prelude::*, winit::WinitSettings};
use resources::game::GameState;
use systems::{dice_system::selected_dice_system, option_system::roll_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .insert_state(GameState::Play)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                selected_dice_system,
                roll_system.run_if(in_state(GameState::Roll)),
            ),
        )
        .run();
}
