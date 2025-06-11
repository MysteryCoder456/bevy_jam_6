use crate::{
    GameAssets,
    screens::{
        Screen,
        level::{GameLayer, Item, player::Player, shopper::Shopper},
    },
};
use avian2d::prelude::*;
use bevy::{color::palettes::css::*, prelude::*};
use std::f32::consts::FRAC_PI_2;

pub enum ShelfOrientation {
    Horizontal,
    Vertical,
}

#[derive(Event)]
pub struct SpawnShelf {
    pub position: Vec2,
    pub orientation: ShelfOrientation,
    pub main_item: Item,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Shelf {
    pub main_item: Item,
}

#[derive(Component)]
struct ShopperSensor;

pub fn plugin(app: &mut App) {
    // Register necessary types
    app.register_type::<Shelf>();

    // Register events
    app.add_event::<SpawnShelf>();

    // (De)spawn systems
    app.add_systems(
        Update,
        (spawn_shelves.run_if(on_event::<SpawnShelf>),).run_if(in_state(Screen::Level)),
    );
    app.add_systems(OnExit(Screen::Level), despawn_shelves);
}

fn spawn_shelves(
    mut commands: Commands,
    mut events: EventReader<SpawnShelf>,
    assets: Res<GameAssets>,
) {
    let shelf_size = Vec2::new(300.0, 80.0);
    let sensor_size = Vec2::new(0.85 * shelf_size.x, 0.4 * shelf_size.y);

    for event in events.read() {
        commands
            .spawn((
                Name::new("Shelf"),
                Shelf {
                    main_item: event.main_item,
                },
                Sprite::from_color(SLATE_GRAY, shelf_size),
                Transform {
                    translation: event.position.extend(0.0),
                    rotation: Quat::from_rotation_z(match event.orientation {
                        ShelfOrientation::Horizontal => 0.0,
                        ShelfOrientation::Vertical => -FRAC_PI_2,
                    }),
                    ..Default::default()
                },
                RigidBody::Static,
                Collider::rectangle(shelf_size.x, shelf_size.y),
                CollisionLayers::new(GameLayer::Shelf, [GameLayer::Player, GameLayer::NPC]),
                Text2d::new(event.main_item.to_string()),
                TextFont {
                    font: assets.game_font.clone(),
                    font_size: 32.0,
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                // Shopper Sensors
                parent
                    .spawn((
                        Name::new("Shopper Sensor 1"),
                        ShopperSensor,
                        Transform::from_xyz(0.0, (shelf_size.y + sensor_size.y) / 2.0, 0.0),
                        Sensor,
                        Collider::rectangle(sensor_size.x, sensor_size.y),
                        CollisionLayers::new(GameLayer::Shelf, [GameLayer::Player, GameLayer::NPC]),
                        CollisionEventsEnabled,
                        // Sprite::from_color(LIGHT_BLUE.with_alpha(0.5), sensor_size),
                    ))
                    .observe(shopper_approached_shelf)
                    .observe(shopper_departed_shelf);
                parent
                    .spawn((
                        Name::new("Shopper Sensor 2"),
                        ShopperSensor,
                        Transform::from_xyz(0.0, -(shelf_size.y + sensor_size.y) / 2.0, 0.0),
                        Sensor,
                        Collider::rectangle(sensor_size.x, sensor_size.y),
                        CollisionLayers::new(GameLayer::Shelf, [GameLayer::Player, GameLayer::NPC]),
                        CollisionEventsEnabled,
                        // Sprite::from_color(LIGHT_BLUE.with_alpha(0.5), sensor_size),
                    ))
                    .observe(shopper_approached_shelf)
                    .observe(shopper_departed_shelf);

                // Collider to prevent NPC shoppers getting stuck on shelf sides
                // parent.spawn((
                //     Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)),
                //     Collider::capsule(shelf_size.y / 2.0, shelf_size.x),
                //     CollisionLayers::new(GameLayer::Shelf, [GameLayer::NPC]),
                //     Friction::new(0.0),
                // ));
            });
    }
}

fn despawn_shelves(mut commands: Commands, shelf_query: Query<Entity, With<Shelf>>) {
    for shelf_entity in shelf_query {
        commands.entity(shelf_entity).despawn();
    }
}

fn shopper_approached_shelf(
    trigger: Trigger<OnCollisionStart>,
    child_of_query: Query<&ChildOf>,
    mut player_query: Query<&mut Player>,
    mut shopper_query: Query<&mut Shopper>,
) {
    let sensor_entity = trigger.target();

    if let Ok(sensor_child_of) = child_of_query.get(sensor_entity) {
        let shelf_entity = sensor_child_of.parent();

        if let Ok(mut player) = player_query.get_mut(trigger.collider) {
            log::debug!("Player has approached shelf sensor {}", sensor_entity);
            player.current_shelf = Some(shelf_entity);
        } else if let Ok(mut shopper) = shopper_query.get_mut(trigger.collider) {
            log::debug!(
                "Shopper {} has approached shelf sensor {}",
                trigger.collider,
                sensor_entity
            );
            shopper.current_shelf = Some(shelf_entity);
        }
    }
}

fn shopper_departed_shelf(
    trigger: Trigger<OnCollisionEnd>,
    mut player_query: Query<&mut Player>,
    mut shopper_query: Query<&mut Shopper>,
) {
    if let Ok(mut player) = player_query.get_mut(trigger.collider) {
        log::debug!("Player has departed shelf sensor {}", trigger.target());
        player.current_shelf = None;
    } else if let Ok(mut shopper) = shopper_query.get_mut(trigger.collider) {
        log::debug!(
            "Shopper {} has departed shelf sensor {}",
            trigger.collider,
            trigger.target()
        );
        shopper.current_shelf = None;
    }
}
