mod game_over;
mod level;
mod main_menu;
mod win;

use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum Screen {
    #[default]
    Loading,
    MainMenu,
    Level,
    Win,
    GameOver,
}

pub fn plugin(app: &mut App) {
    // Add respective screen plugins
    app.add_plugins((
        main_menu::plugin,
        level::plugin,
        win::plugin,
        game_over::plugin,
    ));

    // Initialize Screen state
    app.init_state::<Screen>();
}
