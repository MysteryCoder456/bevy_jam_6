mod checkout;
mod player;
mod shelf;
mod shopper;

use std::collections::HashMap;

use crate::{GameAssets, screens::Screen};
use avian2d::prelude::*;
use bevy::{color::palettes::css::*, prelude::*};
use bevy_enhanced_input::prelude::*;
use checkout::SpawnCheckoutCounter;
use player::Player;
use shelf::SpawnShelf;
use shopper::SpawnShopper;

#[derive(Clone, Copy)]
enum EntityOrientation {
    Horizontal,
    Vertical,
}

impl Into<Quat> for EntityOrientation {
    fn into(self) -> Quat {
        let rotation_z = match self {
            EntityOrientation::Horizontal => 0.0,
            EntityOrientation::Vertical => -std::f32::consts::FRAC_PI_2,
        };
        Quat::from_rotation_z(rotation_z)
    }
}

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
    Environment,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Reflect)]
enum Item {
    ToiletPaper,
    CannedTuna,
    InstantRamen,
    Soap,
}

impl std::fmt::Display for Item {
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

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct Objectives {
    items: HashMap<Item, u32>,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct GameTimer(Timer);

#[derive(Component)]
struct GameTimerUI;

pub fn plugin(app: &mut App) {
    // Register necessary types
    app.register_type::<Item>();
    app.register_type::<Inventory>();
    app.register_type::<Objectives>();
    app.register_type::<GameTimer>();

    // Add debug input actions
    #[cfg(feature = "dev")]
    app.add_input_context::<DebugLevelContext>();

    // Add resources
    app.insert_resource(Objectives {
        items: HashMap::from([
            (Item::ToiletPaper, 12),
            (Item::CannedTuna, 8),
            (Item::Soap, 9),
        ]),
    });
    app.insert_resource(GameTimer(Timer::from_seconds(2.0 * 60.0, TimerMode::Once)));

    // Add game element plugins
    app.add_plugins((
        player::plugin,
        shelf::plugin,
        shopper::plugin,
        checkout::plugin,
    ));

    // Gameplay systems
    app.add_systems(OnEnter(Screen::Level), (spawn_level, spawn_game_timer_ui));
    app.add_systems(OnExit(Screen::Level), despawn_game_timer_ui);
    app.add_systems(
        Update,
        (objectives_fulfilled, game_timer).run_if(in_state(Screen::Level)),
    );

    // Add debug systems
    cfg_if::cfg_if! {
        if #[cfg(feature = "dev")] {
            app.add_systems(OnEnter(Screen::Level), add_debug_actions);
            app.add_systems(OnExit(Screen::Level), remove_debug_actions);

            app.add_observer(spawn_vertical_shelf);
            app.add_observer(spawn_horizontal_shelf);
        }
    }
}

fn spawn_level(
    mut shelf_events: EventWriter<SpawnShelf>,
    mut shopper_events: EventWriter<SpawnShopper>,
    mut checkout_counter_events: EventWriter<SpawnCheckoutCounter>,
) {
    // Spawn a demo scene

    shelf_events.write_batch([
        SpawnShelf {
            position: Vec2::new(0.0, 600.0),
            orientation: EntityOrientation::Horizontal,
            main_item: Item::InstantRamen,
        },
        SpawnShelf {
            position: Vec2::new(0.0, 350.0),
            orientation: EntityOrientation::Horizontal,
            main_item: Item::ToiletPaper,
        },
        SpawnShelf {
            position: Vec2::new(0.0, 100.0),
            orientation: EntityOrientation::Horizontal,
            main_item: Item::Soap,
        },
        SpawnShelf {
            position: Vec2::new(0.0, -150.0),
            orientation: EntityOrientation::Horizontal,
            main_item: Item::CannedTuna,
        },
        SpawnShelf {
            position: Vec2::new(0.0, -400.0),
            orientation: EntityOrientation::Horizontal,
            main_item: Item::ToiletPaper,
        },
    ]);

    shopper_events.write_batch([
        SpawnShopper {
            position: Vec2::new(300.0, 100.0),
        },
        SpawnShopper {
            position: Vec2::new(-300.0, -100.0),
        },
    ]);

    checkout_counter_events.write_batch([
        SpawnCheckoutCounter {
            position: Vec2::new(600.0, -300.0),
            orientation: EntityOrientation::Horizontal,
        },
        SpawnCheckoutCounter {
            position: Vec2::new(600.0, 0.0),
            orientation: EntityOrientation::Horizontal,
        },
        SpawnCheckoutCounter {
            position: Vec2::new(600.0, 300.0),
            orientation: EntityOrientation::Horizontal,
        },
    ]);
}

fn spawn_game_timer_ui(mut commands: Commands, timer: Res<GameTimer>, assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("Game Timer UI"),
        GameTimerUI,
        Text::new(format!("{}s", timer.0.remaining().as_secs())),
        TextColor(WHITE.into()),
        TextFont {
            font: assets.ui_font.clone(),
            font_size: 18.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        BackgroundColor(BLACK.with_alpha(0.6).into()),
        BorderRadius::new(Val::ZERO, Val::ZERO, Val::ZERO, Val::Px(8.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::ZERO,
            right: Val::ZERO,
            display: Display::Flex,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(4.0)),
            min_width: Val::Px(50.0),
            ..Default::default()
        },
    ));
}

fn despawn_game_timer_ui(mut commands: Commands, query: Single<Entity, With<GameTimerUI>>) {
    commands.entity(query.entity()).despawn();
}

fn objectives_fulfilled(
    objectives: Res<Objectives>,
    inventory: Single<&Inventory, (With<Player>, Changed<Inventory>)>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    // Check if the player has collected all required items
    let all_items_collected = objectives.items.iter().all(|(item, &required_count)| {
        inventory
            .0
            .get(item)
            .map_or(false, |&count| count >= required_count)
    });

    if all_items_collected {
        next_screen.set(Screen::Win);
    }
}

fn game_timer(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut query: Single<&mut Text, With<GameTimerUI>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    timer.0.tick(time.delta());
    query.0 = format!("{}s", timer.0.remaining().as_secs());

    if timer.0.just_finished() {
        next_screen.set(Screen::GameOver);
    }
}

#[cfg(feature = "dev")]
fn create_debug_actions() -> Actions<DebugLevelContext> {
    let mut actions = Actions::<DebugLevelContext>::default();
    actions
        .bind::<DebugSpawnShelfVertical>()
        .to(KeyCode::KeyV)
        .with_conditions(Press::default());
    actions
        .bind::<DebugSpawnShelfHorizontal>()
        .to(KeyCode::KeyH)
        .with_conditions(Press::default());
    actions
}

#[cfg(feature = "dev")]
fn add_debug_actions(mut commands: Commands) {
    let actions = create_debug_actions();
    commands.spawn(actions);
}

#[cfg(feature = "dev")]
fn remove_debug_actions(
    mut commands: Commands,
    query: Single<Entity, With<Actions<DebugLevelContext>>>,
) {
    commands.entity(query.entity()).despawn();
}

#[cfg(feature = "dev")]
fn spawn_vertical_shelf(
    _trigger: Trigger<Fired<DebugSpawnShelfVertical>>,
    mut shelf_events: EventWriter<SpawnShelf>,
    player_query: Single<&Transform, With<Player>>,
) {
    let offset = Vec2::new(100.0, 0.0);
    shelf_events.write(SpawnShelf {
        position: player_query.translation.truncate() + offset,
        orientation: EntityOrientation::Vertical,
        main_item: Item::ToiletPaper,
    });
}

#[cfg(feature = "dev")]
fn spawn_horizontal_shelf(
    _trigger: Trigger<Fired<DebugSpawnShelfHorizontal>>,
    mut shelf_events: EventWriter<SpawnShelf>,
    player_query: Single<&Transform, With<Player>>,
) {
    let offset = Vec2::new(0.0, 100.0);
    shelf_events.write(SpawnShelf {
        position: player_query.translation.truncate() + offset,
        orientation: EntityOrientation::Horizontal,
        main_item: Item::ToiletPaper,
    });
}
