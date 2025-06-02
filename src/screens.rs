mod level;
mod main_menu;

use bevy::prelude::*;

// TODO: Change default back to MainMenu when game is done
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum Screen {
    MainMenu,
    #[default]
    Level,
}

pub fn plugin(app: &mut App) {
    // Add respective screen plugins
    app.add_plugins((main_menu::plugin, level::plugin));

    // Initialize Screen state
    app.init_state::<Screen>();
}
