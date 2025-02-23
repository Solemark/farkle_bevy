mod resources;
mod systems;

use crate::{
    resources::game::GameState,
    systems::{
        check::check_system, dice::dice_buttons, farkle::farkle_system, new_turn::new_turn,
        options::option_buttons, roll::roll, setup::setup, status::status_system,
    },
};
use bevy::{prelude::*, winit::WinitSettings};
use systems::score::{display_score, scoring_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .insert_state(GameState::Roll)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                status_system,
                dice_buttons,
                display_score,
                scoring_system,
                roll.run_if(in_state(GameState::Roll)),
                option_buttons.run_if(in_state(GameState::Play)),
                check_system.run_if(in_state(GameState::Check)),
                farkle_system.run_if(in_state(GameState::Farkle)),
                new_turn.run_if(in_state(GameState::EndTurn)),
            ),
        )
        .run();
}
