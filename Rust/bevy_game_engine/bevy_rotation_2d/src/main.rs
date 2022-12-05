/**
 * original => https://github.com/bevyengine/bevy/blob/latest/examples/2d/rotation.rs
 */

use bevy_rotation_2d::{*, constants::TIME_STEP};
use bevy::{prelude::{App, SystemSet}, DefaultPlugins, time::FixedTimestep};

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::load)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(keyboard::load)
                .with_system(enemy::snap_to_player)
                .with_system(enemy::rotate_to_player_system)
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}
