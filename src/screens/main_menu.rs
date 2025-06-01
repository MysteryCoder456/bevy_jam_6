use super::Screen;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::MainMenu), spawn_main_menu)
        .add_systems(OnExit(Screen::MainMenu), despawn_main_menu);
}

fn spawn_main_menu() {
    // TODO: spawn main menu UI elements
}

fn despawn_main_menu() {
    // TODO: despawn main menu UI elements
}
