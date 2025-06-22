use crate::{
    GameAssets,
    screens::{
        Screen,
        level::{
            GameLayer, Inventory,
            player::{Player, PlayerPickedItem},
            shelf::Shelf,
        },
    },
};
use avian2d::prelude::*;
use bevy::{color::palettes::css::*, prelude::*};

const PANIC_THRESHOLD: u32 = 5;
const PANIC_DISTANCE: f32 = 300.0;

#[derive(Event)]
pub struct SpawnShopper {
    pub position: Vec2,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Shopper {
    pub current_shelf: Option<Entity>,
    panic_meter: u32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
enum ShopperState {
    Wandering {
        timer: Timer,
        direction: Vec2,
    },
    Traveling {
        target_shelf: Entity,
    },
    Taking {
        timer: Timer,
        taking_timer: Timer,
        target_shelf: Entity,
    },
    Panicked,
}

#[derive(Component)]
struct PanicMeterIndicator;

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
            panic_meter_indicator,
            shopper_state_machine,
            shopper_wandering.after(shopper_state_machine),
            shopper_traveling.after(shopper_state_machine),
            shopper_taking.after(shopper_state_machine),
            shopper_panicked.after(shopper_state_machine),
        )
            .run_if(in_state(Screen::Level)),
    );
    app.add_systems(OnExit(Screen::Level), despawn_shoppers);

    // Add observers
    app.add_observer(panic_meter);
}

fn spawn_shoppers(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventReader<SpawnShopper>,
) {
    let shopper_size = Vec2::new(50.0, 25.0);

    for event in events.read() {
        commands.spawn((
            Name::new("Shopper"),
            Shopper {
                current_shelf: None,
                panic_meter: 0,
            },
            ShopperState::Wandering {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
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
            CollisionLayers::new(
                GameLayer::NPC,
                [
                    GameLayer::Default,
                    GameLayer::Player,
                    GameLayer::NPC,
                    GameLayer::Shelf,
                ],
            ),
            CollisionEventsEnabled,
            LinearDamping(1.2),
            AngularDamping(2.0),
            MaxLinearSpeed(100.0),
            ExternalImpulse::default().with_persistence(false),
            children![(
                Name::new("Panic Meter"),
                PanicMeterIndicator,
                Text::new(""),
                TextColor(RED.into()),
                TextFont {
                    font: assets.game_font.clone(),
                    font_size: 24.0,
                    ..Default::default()
                },
                TextLayout {
                    justify: JustifyText::Center,
                    ..Default::default()
                },
            )],
        ));
    }
}

fn despawn_shoppers(mut commands: Commands, query: Query<Entity, With<Shopper>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn panic_meter(
    _trigger: Trigger<PlayerPickedItem>,
    mut shopper_query: Query<(&Transform, &mut Shopper)>,
    player_query: Single<&Transform, With<Player>>,
) {
    shopper_query
        .par_iter_mut()
        .for_each(|(shopper_transform, mut shopper)| {
            let distance = shopper_transform
                .translation
                .truncate()
                .distance(player_query.translation.truncate());

            if distance <= PANIC_DISTANCE {
                shopper.panic_meter += 1;
            }
        });
}

fn panic_meter_indicator(
    shopper_query: Query<(&Shopper, &Children), Changed<Shopper>>,
    mut panic_indicator_query: Query<&mut Text, With<PanicMeterIndicator>>,
) {
    for (shopper, shopper_children) in shopper_query.iter() {
        let indicator = shopper_children
            .iter()
            .filter(|e| panic_indicator_query.contains(*e))
            .next()
            .map(|e| panic_indicator_query.get_mut(e).unwrap());
        if let Some(mut indicator) = indicator {
            indicator.0 = "!".repeat(shopper.panic_meter as usize);
        }
    }
}

fn shopper_wandering(
    mut query: Query<(&mut Transform, &mut ExternalImpulse, &ShopperState), With<Shopper>>,
) {
    let wander_impulse = 400.0;

    query
        .par_iter_mut()
        .for_each(|(mut transform, mut impulse, shopper_state)| {
            if let ShopperState::Wandering {
                timer: _,
                direction,
            } = *shopper_state
            {
                transform.rotation = Quat::from_rotation_z(direction.to_angle());
                impulse.apply_impulse(direction * wander_impulse);
            }
        });
}

fn shopper_traveling(
    mut shopper_query: Query<(&mut Transform, &mut ExternalImpulse, &ShopperState), With<Shopper>>,
    shelf_query: Query<&Transform, (With<Shelf>, Without<Shopper>)>,
) {
    let travel_impulse = 1000.0;

    shopper_query.par_iter_mut().for_each(
        |(mut shopper_transform, mut shopper_impulse, shopper_state)| {
            if let ShopperState::Traveling { target_shelf } = *shopper_state {
                if let Ok(shelf_transform) = shelf_query.get(target_shelf) {
                    // Calculate direction to the target shelf
                    let direction = (shelf_transform.translation - shopper_transform.translation)
                        .truncate()
                        .normalize_or_zero();

                    shopper_transform.rotation = Quat::from_rotation_z(direction.to_angle());
                    shopper_impulse.apply_impulse(direction * travel_impulse);
                }
            }
        },
    );
}

fn shopper_taking(
    time: Res<Time>,
    mut shopper_query: Query<(&mut Inventory, &mut ShopperState), With<Shopper>>,
    shelf_query: Query<&Shelf>,
) {
    shopper_query
        .par_iter_mut()
        .for_each(|(mut inventory, mut shopper_state)| {
            if let ShopperState::Taking {
                timer: _,
                ref mut taking_timer,
                target_shelf,
            } = *shopper_state
            {
                if let Ok(shelf) = shelf_query.get(target_shelf) {
                    if taking_timer.tick(time.delta()).just_finished() {
                        // Add the shelf's main item to the shopper's inventory
                        inventory
                            .0
                            .entry(shelf.main_item)
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                    }
                }
            }
        });
}

fn shopper_panicked(
    mut shopper_query: Query<(&mut Transform, &mut ExternalImpulse, &ShopperState), With<Shopper>>,
    player_query: Single<&Transform, (With<Player>, Without<Shopper>)>,
) {
    let panic_impulse = 5000.0;

    shopper_query.par_iter_mut().for_each(
        |(mut shopper_transform, mut shopper_impulse, shopper_state)| {
            if let ShopperState::Panicked = *shopper_state {
                // Calculate direction towards the player
                let direction = (player_query.translation - shopper_transform.translation)
                    .truncate()
                    .normalize_or_zero();

                // Stampede the player lmao
                shopper_transform.rotation = Quat::from_rotation_z(direction.to_angle());
                shopper_impulse.apply_impulse(direction * panic_impulse);
            }
        },
    );
}

fn shopper_state_machine(
    time: Res<Time>,
    mut shopper_query: Query<(&Transform, &Shopper, &mut ShopperState)>,
    shelf_query: Query<(Entity, &Transform), With<Shelf>>,
) {
    shopper_query
        .par_iter_mut()
        .for_each(|(shopper_transform, shopper, mut shopper_state)| {
            match *shopper_state {
                ShopperState::Wandering {
                    ref mut timer,
                    direction: _,
                } => {
                    // If shopper is about to panic, just panic
                    if shopper.panic_meter >= PANIC_THRESHOLD {
                        // Transition to panicked state
                        *shopper_state = ShopperState::Panicked;
                        return;
                    }

                    if !timer.tick(time.delta()).just_finished() {
                        // Continue wandering
                        return;
                    }

                    // Choose a random shelf to travel to
                    let target_shelf = {
                        // Choose a random shelf from the 5 closests shelves
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
                        fastrand::choice(closest_shelves.iter().take(5)).unwrap().0
                    };

                    // Transition to traveling to the random shelf
                    *shopper_state = ShopperState::Traveling { target_shelf };
                }
                ShopperState::Traveling { target_shelf } => {
                    // Transition to taking from the shelf if reached the target shelf
                    if shopper.current_shelf.is_some_and(|s| s == target_shelf) {
                        *shopper_state = ShopperState::Taking {
                            timer: Timer::from_seconds(8.0, TimerMode::Once),
                            taking_timer: Timer::from_seconds(1.5, TimerMode::Repeating),
                            target_shelf,
                        };
                    }
                }
                ShopperState::Taking {
                    ref mut timer,
                    taking_timer: _,
                    target_shelf: _,
                } => {
                    if !timer.tick(time.delta()).just_finished() {
                        // Continue taking
                        return;
                    }

                    // Chose a random direction to wander off towards
                    let wander_direction =
                        Vec2::new(fastrand::f32() * 2.0 - 1.0, fastrand::f32() * 2.0 - 1.0)
                            .normalize_or_zero();

                    *shopper_state = ShopperState::Wandering {
                        timer: Timer::from_seconds(10.0, TimerMode::Once),
                        direction: wander_direction,
                    };
                }
                ShopperState::Panicked => {
                    // Do nothing, stay in panic state.
                }
            }
        });
}
