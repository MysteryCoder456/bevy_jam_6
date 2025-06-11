mod player;
mod shelf;
mod shopper;

use std::{collections::HashMap, fmt::Display};

use crate::screens::Screen;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use player::Player;
use shelf::{ShelfOrientation, SpawnShelf};
use shopper::SpawnShopper;

#[cfg(feature = "dev")]
#[derive(InputContext)]
struct DebugLevelContext;

#[cfg(feature = "dev")]
#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct DebugSpawnShelfVertical;

#[cfg(feature = "dev")]
#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct DebugSpawnShelfHorizontal;

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Default,
    Player,
    NPC,
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

    // Add debug input actions
    cfg_if::cfg_if! {
        if #[cfg(feature = "dev")] {
            let mut actions = Actions::<DebugLevelContext>::default();
            actions
                .bind::<DebugSpawnShelfVertical>()
                .to(KeyCode::KeyV)
                .with_conditions(Press::default());
            actions
                .bind::<DebugSpawnShelfHorizontal>()
                .to(KeyCode::KeyH)
                .with_conditions(Press::default());
        }
    }

    // Add game element plugins
    app.add_plugins((player::plugin, shelf::plugin, shopper::plugin));

    // Gameplay systems
    app.add_systems(OnEnter(Screen::Level), spawn_level);

    // Add debug systems
    cfg_if::cfg_if! {
        if #[cfg(feature = "dev")] {
            app.add_observer(spawn_vertical_shelf);
            app.add_observer(spawn_horizontal_shelf);
        }
    }
}

fn spawn_level(
    mut shelf_events: EventWriter<SpawnShelf>,
    mut shopper_events: EventWriter<SpawnShopper>,
) {
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

    shopper_events.write_batch([
        SpawnShopper {
            position: Vec2::new(100.0, 100.0),
        },
        SpawnShopper {
            position: Vec2::new(-100.0, -100.0),
        },
    ]);
}

fn spawn_vertical_shelf(
    _trigger: Trigger<Fired<DebugSpawnShelfVertical>>,
    mut shelf_events: EventWriter<SpawnShelf>,
    player_query: Single<&Transform, With<Player>>,
) {
    let offset = Vec2::new(0.0, 50.0);
    shelf_events.write(SpawnShelf {
        position: player_query.translation.truncate() + offset,
        orientation: ShelfOrientation::Vertical,
        main_item: Item::ToiletPaper,
    });
}

fn spawn_horizontal_shelf(
    _trigger: Trigger<Fired<DebugSpawnShelfHorizontal>>,
    mut shelf_events: EventWriter<SpawnShelf>,
    player_query: Single<&Transform, With<Player>>,
) {
    let offset = Vec2::new(50.0, 0.0);
    shelf_events.write(SpawnShelf {
        position: player_query.translation.truncate() + offset,
        orientation: ShelfOrientation::Horizontal,
        main_item: Item::ToiletPaper,
    });
}
