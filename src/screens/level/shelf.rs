use bevy::{color::palettes::css::*, prelude::*};
use std::f32::consts::FRAC_PI_2;

use crate::screens::Screen;

pub enum ShelfOrientation {
    Horizontal,
    Vertical,
}

#[derive(Event)]
pub struct SpawnShelf {
    pub position: Vec2,
    pub orientation: ShelfOrientation,
}

#[derive(Component)]
struct Shelf;

pub fn plugin(app: &mut App) {
    // Register events
    app.add_event::<SpawnShelf>();

    // Spawn systems
    app.add_systems(
        Update,
        (spawn_shelves.run_if(on_event::<SpawnShelf>),).run_if(in_state(Screen::Level)),
    );
}

fn spawn_shelves(mut commands: Commands, mut events: EventReader<SpawnShelf>) {
    for event in events.read() {
        commands.spawn((
            Name::new("Shelf"),
            Shelf,
            Sprite::from_color(SLATE_GRAY, Vec2::new(300.0, 80.0)),
            Transform {
                translation: event.position.extend(0.0),
                rotation: Quat::from_rotation_z(match event.orientation {
                    ShelfOrientation::Horizontal => 0.0,
                    ShelfOrientation::Vertical => FRAC_PI_2,
                }),
                ..Default::default()
            },
        ));
    }
}
