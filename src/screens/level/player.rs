use std::ops::{Add, AddAssign};

use crate::screens::{
    Screen,
    level::{GameLayer, Inventory, shelf::Shelf},
};
use avian2d::prelude::*;
use bevy::{color::palettes::css::*, prelude::*};
use bevy_enhanced_input::{prelude::*, preset::Bidirectional};

const LINEAR_ACCELERATION: f32 = 3.0;
const STEER_ACCELERATION: f32 = 0.1;

#[derive(InputContext)]
struct PlayerInputContext;

#[derive(Debug, InputAction)]
#[input_action(output = f32)]
struct Accelerate;

#[derive(Debug, InputAction)]
#[input_action(output = f32)]
struct Steer;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Interact;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub current_shelf: Option<Entity>,
}

pub fn plugin(app: &mut App) {
    // Register necessary types
    app.register_type::<Player>();
    app.add_input_context::<PlayerInputContext>();

    // Screen enter and exit systems
    app.add_systems(OnEnter(Screen::Level), spawn_player);
    app.add_systems(OnEnter(Screen::Level), despawn_player);

    // Player input reactions
    app.add_observer(player_acceleration);
    app.add_observer(player_steering);
    app.add_observer(player_interaction);
}

fn spawn_player(mut commands: Commands) {
    // Bind inputs to actions
    let mut actions = Actions::<PlayerInputContext>::default();
    actions
        .bind::<Accelerate>()
        .to((
            Bidirectional {
                positive: KeyCode::KeyW,
                negative: KeyCode::KeyS,
            },
            Bidirectional {
                positive: KeyCode::ArrowUp,
                negative: KeyCode::ArrowDown,
            },
            GamepadAxis::LeftStickY,
        ))
        .with_modifiers(DeadZone::default());
    actions
        .bind::<Steer>()
        .to((
            Bidirectional {
                positive: KeyCode::KeyA,
                negative: KeyCode::KeyD,
            },
            Bidirectional {
                positive: KeyCode::ArrowLeft,
                negative: KeyCode::ArrowRight,
            },
            GamepadAxis::LeftStickX.with_modifiers(Negate::all()),
        ))
        .with_modifiers(DeadZone::default());
    actions
        .bind::<Interact>()
        .to((KeyCode::Space, GamepadButton::South));

    // Spawn player
    let player_size = Vec2::new(50.0, 25.0);
    commands.spawn((
        Name::new("Player"),
        Player {
            current_shelf: None,
        },
        Inventory::default(),
        Sprite::from_color(LIMEGREEN, player_size),
        RigidBody::Dynamic,
        Collider::rectangle(player_size.x, player_size.y),
        CollisionLayers::new(GameLayer::Shopper, [GameLayer::Shopper, GameLayer::Shelf]),
        LinearDamping(1.2),
        AngularDamping(2.0),
        actions,
    ));
}

fn despawn_player(mut commands: Commands, player_query: Single<Entity, With<Player>>) {
    commands.entity(player_query.entity()).despawn();
}

fn player_acceleration(
    trigger: Trigger<Fired<Accelerate>>,
    mut query: Query<(&Transform, &mut LinearVelocity), With<Player>>,
) {
    if let Ok((transform, mut linear_velocity)) = query.get_mut(trigger.target()) {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let acceleration_mag = LINEAR_ACCELERATION * trigger.value;
        let acceleration = acceleration_mag * Vec2::from_angle(angle);
        linear_velocity.0 += acceleration;
    }
}

fn player_steering(
    trigger: Trigger<Fired<Steer>>,
    mut query: Query<&mut AngularVelocity, With<Player>>,
) {
    if let Ok(mut angular_velocity) = query.get_mut(trigger.target()) {
        angular_velocity.0 += trigger.value * STEER_ACCELERATION;
    }
}

fn player_interaction(
    trigger: Trigger<Fired<Interact>>,
    mut player_query: Query<(&Player, &mut Inventory)>,
    shelf_query: Query<&Shelf>,
) {
    if let Ok((player, mut player_inventory)) = player_query.get_mut(trigger.target()) {
        if let Some(shelf_entity) = player.current_shelf {
            if let Ok(shelf) = shelf_query.get(shelf_entity) {
                if let Some(main_item_quantity) = player_inventory.0.get_mut(&shelf.main_item) {
                    *main_item_quantity = *main_item_quantity + 1;
                } else {
                    player_inventory.0.insert(shelf.main_item, 1);
                }
            }
        }
    }
}
