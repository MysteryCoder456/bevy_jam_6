use crate::screens::{
    Screen,
    level::{EntityOrientation, GameLayer},
};
use avian2d::prelude::*;
use bevy::{color::palettes::css::*, prelude::*};

#[derive(Event)]
pub struct SpawnCheckoutCounter {
    pub position: Vec2,
    pub orientation: EntityOrientation,
}

#[derive(Component)]
struct CheckoutCounter;

pub fn plugin(app: &mut App) {
    // Register events
    app.add_event::<SpawnCheckoutCounter>();

    // Checkout counter systems
    app.add_systems(
        Update,
        spawn_checkout_counters
            .run_if(in_state(Screen::Level).and(on_event::<SpawnCheckoutCounter>)),
    );
    app.add_systems(OnExit(Screen::Level), despawn_checkout_counters);
}

fn spawn_checkout_counters(mut commands: Commands, mut events: EventReader<SpawnCheckoutCounter>) {
    let counter_size = Vec2::new(140.0, 60.0);

    for event in events.read() {
        commands.spawn((
            Name::new("Checkout Counter"),
            CheckoutCounter,
            Sprite::from_color(ORANGE_RED, counter_size),
            Transform {
                translation: event.position.extend(0.0),
                rotation: event.orientation.into(),
                ..Default::default()
            },
            RigidBody::Static,
            Collider::rectangle(counter_size.x, counter_size.y),
            CollisionLayers::new(GameLayer::Environment, [GameLayer::Player, GameLayer::NPC]),
        ));
    }
}

fn despawn_checkout_counters(mut commands: Commands, query: Query<Entity, With<CheckoutCounter>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
