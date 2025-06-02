use crate::screens::Screen;
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

#[derive(Component)]
struct Player;

pub fn plugin(app: &mut App) {
    // Register input contexts
    app.add_input_context::<PlayerInputContext>();

    // Screen enter and exit systems
    app.add_systems(OnEnter(Screen::Level), spawn_player);
    app.add_systems(OnEnter(Screen::Level), despawn_player);

    // Player input reactions
    app.add_observer(player_acceleration);
    app.add_observer(player_steering);
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
            GamepadAxis::LeftStickX.with_modifiers(Negate::all()),
        ))
        .with_modifiers(DeadZone::default());

    // Spawn player
    let player_size = Vec2::new(50.0, 25.0);
    commands.spawn((
        Name::new("Player"),
        Player,
        Sprite::from_color(LIMEGREEN, player_size),
        RigidBody::Dynamic,
        Collider::rectangle(player_size.x, player_size.y),
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
