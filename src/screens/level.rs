mod player;
mod shelf;

use crate::screens::Screen;
use bevy::prelude::*;
use shelf::{ShelfOrientation, SpawnShelf};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Velocity {
    linear: Vec2,
    angular: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct VelocityDamping {
    linear: f32,
    angular: f32,
}

pub fn plugin(app: &mut App) {
    // Register types for inspector
    app.register_type::<Velocity>();
    app.register_type::<VelocityDamping>();

    // Add game element plugins
    app.add_plugins((player::plugin, shelf::plugin));

    // Screen enter and exit systems
    app.add_systems(OnEnter(Screen::Level), spawn_level);
    app.add_systems(OnExit(Screen::Level), despawn_level);

    app.add_systems(
        FixedUpdate,
        (velocity_system, velocity_damping_system)
            .chain()
            .run_if(in_state(Screen::Level)),
    );
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

fn velocity_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let dt = time.delta_secs();

    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.linear.extend(0.0) * dt;
        transform.rotate_z(velocity.angular * dt);
    });
}

fn velocity_damping_system(time: Res<Time>, mut query: Query<(&mut Velocity, &VelocityDamping)>) {
    let dt = time.delta_secs();

    query.par_iter_mut().for_each(|(mut velocity, damping)| {
        velocity.linear *= 1.0 - damping.linear * dt;
        velocity.angular *= 1.0 - damping.angular * dt;
    });
}
