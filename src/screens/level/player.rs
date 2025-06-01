use crate::screens::Screen;
use bevy::{color::palettes::css::*, prelude::*};

#[derive(Component)]
struct Player;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Level), spawn_player);
    app.add_systems(OnEnter(Screen::Level), despawn_player);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        Player,
        Sprite::from_color(GREEN, Vec2::new(50., 25.)),
    ));
}

fn despawn_player(mut commands: Commands, player_query: Single<Entity, With<Player>>) {
    commands.entity(player_query.entity()).despawn();
}
