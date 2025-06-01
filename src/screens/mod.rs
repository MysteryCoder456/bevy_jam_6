mod main_menu;

use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum Screen {
    #[default]
    MainMenu,
    Level,
    PauseMenu,
}

pub fn plugin(app: &mut App) {
    // Add respective screen plugins
    app.add_plugins((main_menu::plugin,));

    // Initialize Screen state
    app.init_state::<Screen>();
}
