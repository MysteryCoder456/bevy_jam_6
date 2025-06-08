use std::time::Duration;

use crate::screens::{
    Screen,
    level::{GameLayer, Inventory, shelf::Shelf},
};
use avian2d::prelude::*;
use bevy::{color::palettes::css::*, prelude::*};
use rand::prelude::*;

#[derive(Event)]
pub struct SpawnShopper {
    pub position: Vec2,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Shopper {
    state_timer: Timer,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
enum ShopperState {
    Wandering { direction: Vec2 },
    Traveling { target_shelf: Entity },
    Taking { target_shelf: Entity },
    Panicked,
}

pub fn plugin(app: &mut App) {
    // Register necessary types
    app.register_type::<Shopper>();
    app.register_type::<ShopperState>();

    // Register spawn events
    app.add_event::<SpawnShopper>();

    // Shopper systems
    app.add_systems(
        Update,
        (
            spawn_shoppers.run_if(on_event::<SpawnShopper>),
            shopper_state_machine,
            shopper_wandering.after(shopper_state_machine),
            shopper_traveling.after(shopper_state_machine),
            shopper_taking.after(shopper_state_machine),
            shopper_panicked.after(shopper_state_machine),
        )
            .run_if(in_state(Screen::Level)),
    );
    app.add_systems(OnExit(Screen::Level), despawn_shoppers);
}

fn spawn_shoppers(mut commands: Commands, mut events: EventReader<SpawnShopper>) {
    let shopper_size = Vec2::new(50.0, 25.0);

    for event in events.read() {
        commands.spawn((
            Name::new("Shopper"),
            Shopper {
                state_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            },
            ShopperState::Wandering {
                direction: Vec2::ZERO,
            },
            Inventory::default(),
            Sprite::from_color(YELLOW, shopper_size),
            Transform {
                translation: event.position.extend(0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::rectangle(shopper_size.x, shopper_size.y),
            CollisionLayers::new(GameLayer::Shopper, [GameLayer::Shopper, GameLayer::Shelf]),
            LinearDamping(1.2),
            AngularDamping(2.0),
        ));
    }
}

fn despawn_shoppers(mut commands: Commands, query: Query<Entity, With<Shopper>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn shopper_wandering(
    mut shopper_query: Query<(&mut Transform, &mut LinearVelocity, &ShopperState), With<Shopper>>,
) {
    let wander_speed = 50.0;

    shopper_query
        .par_iter_mut()
        .for_each(|(mut transform, mut velocity, shopper_state)| {
            if let ShopperState::Wandering { direction } = *shopper_state {
                if direction != Vec2::ZERO {
                    // Make the shopper face the direction its wandering in
                    transform.rotation = Quat::from_rotation_z(direction.to_angle());
                }
                velocity.0 = direction * wander_speed;
            }
        });
}

fn shopper_traveling(
    mut shopper_query: Query<(&mut Transform, &mut LinearVelocity, &ShopperState), With<Shopper>>,
    shelf_query: Query<&Transform, (With<Shelf>, Without<Shopper>)>,
) {
    let travel_speed = 80.0;

    shopper_query.par_iter_mut().for_each(
        |(mut shopper_transform, mut shopper_velocity, shopper_state)| {
            if let ShopperState::Traveling { target_shelf } = *shopper_state {
                if let Ok(shelf_transform) = shelf_query.get(target_shelf) {
                    // Calculate direction to the target shelf
                    let direction = (shelf_transform.translation - shopper_transform.translation)
                        .truncate()
                        .normalize_or_zero();

                    // Make the shopper face the direction its wandering in
                    shopper_transform.rotation = Quat::from_rotation_z(direction.to_angle());
                    shopper_velocity.0 = direction * travel_speed;
                }
            }
        },
    );
}

fn shopper_taking() {
    // TODO: implement
}

fn shopper_panicked() {
    // TODO: implement
}

fn shopper_state_machine(
    time: Res<Time>,
    mut shopper_query: Query<(&Transform, &mut Shopper, &mut ShopperState)>,
    shelf_query: Query<(Entity, &Transform), With<Shelf>>,
) {
    shopper_query
        .par_iter_mut()
        .for_each(|(shopper_transform, mut shopper, mut shopper_state)| {
            if !shopper.state_timer.tick(time.delta()).just_finished() {
                return;
            }

            let next_state = match *shopper_state {
                ShopperState::Wandering { direction: _ } => {
                    // Choose a random shelf to travel to
                    let target_shelf = {
                        // Get the five closest shelves from this shopper
                        let mut closest_shelves = shelf_query
                            .iter()
                            .map(|(shelf_entity, shelf_transform)| {
                                let distance = shopper_transform
                                    .translation
                                    .distance_squared(shelf_transform.translation);
                                (shelf_entity, distance)
                            })
                            .collect::<Vec<_>>();
                        closest_shelves.sort_by_key(|(_, distance)| distance.round() as u32);
                        let closest_shelves = closest_shelves.iter().take(5).collect::<Vec<_>>();

                        // Choose a random shelf from the five closest ones
                        let chosen_idx = rand::rng().random_range(0..closest_shelves.len());
                        closest_shelves.get(chosen_idx).unwrap().0
                    };

                    // Transition to traveling to the random shelf
                    shopper.state_timer.set_duration(Duration::from_secs(2));
                    shopper.state_timer.reset();
                    ShopperState::Traveling { target_shelf }

                    // TODO: Alternatively, panic if seeing another shopper take an item
                }
                ShopperState::Traveling { target_shelf } => {
                    // Transition to taking from the shelf
                    // TODO: ONLY if shopper collides with the target shelf
                    shopper.state_timer.set_duration(Duration::from_secs(10));
                    shopper.state_timer.reset();
                    ShopperState::Taking { target_shelf }
                }
                ShopperState::Taking { target_shelf: _ } => {
                    // Chose a random direction to wander off towards
                    let wander_direction = Vec2::new(
                        rand::rng().random_range(-1.0..=1.0),
                        rand::rng().random_range(-1.0..=1.0),
                    )
                    .normalize_or_zero();

                    shopper.state_timer.set_duration(Duration::from_secs(15));
                    shopper.state_timer.reset();
                    ShopperState::Wandering {
                        direction: wander_direction,
                    }
                }
                ShopperState::Panicked => {
                    // Do nothing, stay in panic state.
                    ShopperState::Panicked
                }
            };
            *shopper_state = next_state;
        });
}
