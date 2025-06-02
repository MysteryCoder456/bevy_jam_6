mod player;
mod shelf;

use crate::screens::Screen;
use bevy::prelude::*;
use shelf::{ShelfOrientation, SpawnShelf};

pub fn plugin(app: &mut App) {
    // Add game element plugins
    app.add_plugins((player::plugin, shelf::plugin));

    // Screen enter and exit systems
    app.add_systems(OnEnter(Screen::Level), spawn_level);
    app.add_systems(OnExit(Screen::Level), despawn_level);
}

fn spawn_level(mut shelf_events: EventWriter<SpawnShelf>) {
    // Spawn a demo scene

    shelf_events.write_batch([
        SpawnShelf {
            position: Vec2::new(0.0, 200.0),
            orientation: ShelfOrientation::Horizontal,
        },
        SpawnShelf {
            position: Vec2::new(0.0, -200.0),
            orientation: ShelfOrientation::Horizontal,
        },
        SpawnShelf {
            position: Vec2::new(-300.0, 0.0),
            orientation: ShelfOrientation::Vertical,
        },
    ]);
}

fn despawn_level() {
    // TODO: despawn level elements
}
