mod level;
mod main_menu;

use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum Screen {
    #[default]
    Loading,
    MainMenu,
    Level,
}

pub fn plugin(app: &mut App) {
    // Add respective screen plugins
    app.add_plugins((main_menu::plugin, level::plugin));

    // Initialize Screen state
    app.init_state::<Screen>();
}
