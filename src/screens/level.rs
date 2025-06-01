mod player;

use super::Screen;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    // Add game element plugins
    app.add_plugins((player::plugin,));

    // Screen enter and exit systems
    app.add_systems(OnEnter(Screen::Level), spawn_level);
    app.add_systems(OnExit(Screen::Level), despawn_level);
}

fn spawn_level() {
    // TODO: spawn level elements
}

fn despawn_level() {
    // TODO: despawn level elements
}
