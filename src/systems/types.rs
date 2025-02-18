use crate::resources::game::OptionUI;
use bevy::prelude::*;

/// Option button query params
pub type OBQuery = (With<Button>, With<OptionUI>);
/// Dice button query params
pub type DBQuery = (With<Button>, Without<OptionUI>);
