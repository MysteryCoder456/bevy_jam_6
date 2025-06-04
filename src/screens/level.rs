mod player;
mod shelf;

use std::{collections::HashMap, fmt::Display};

use crate::screens::Screen;
use avian2d::prelude::*;
use bevy::prelude::*;
use shelf::{ShelfOrientation, SpawnShelf};

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Default,
    Shopper,
    Shelf,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Reflect)]
enum Item {
    ToiletPaper,
    CannedTuna,
    InstantRamen,
    Soap,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_repr = match self {
            Self::ToiletPaper => "Toilet Paper",
            Self::CannedTuna => "Canned Tuna",
            Self::InstantRamen => "Instant Ramen",
            Self::Soap => "Soap",
        };
        write!(f, "{}", str_repr)
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Inventory(HashMap<Item, u32>);

pub fn plugin(app: &mut App) {
    // Register necessary types
    app.register_type::<Item>();
    app.register_type::<Inventory>();

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
            main_item: Item::ToiletPaper,
        },
        SpawnShelf {
            position: Vec2::new(0.0, -200.0),
            orientation: ShelfOrientation::Horizontal,
            main_item: Item::InstantRamen,
        },
        SpawnShelf {
            position: Vec2::new(-300.0, 0.0),
            orientation: ShelfOrientation::Vertical,
            main_item: Item::Soap,
        },
    ]);
}

fn despawn_level() {
    // TODO: despawn level elements
}
