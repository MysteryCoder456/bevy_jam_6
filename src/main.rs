// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod screens;

use avian2d::prelude::*;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_enhanced_input::prelude::*;

#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use screens::Screen;

#[derive(Resource)]
struct GameAssets {
    ui_font: Handle<Font>,
    game_font: Handle<Font>,
    shopper_npc: Handle<Image>,
    shopper_player: Handle<Image>,
}

#[derive(Component)]
struct DefaultCamera;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Toilet Paper".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
            EnhancedInputPlugin,
            PhysicsPlugins::default(),
        ));

        // Inspector plugins for dev-builds
        #[cfg(feature = "bevy-inspector-egui")]
        app.add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::new(),
        ));

        // Add game plugins
        app.add_plugins(screens::plugin);

        // Disable gravity
        app.insert_resource(Gravity(Vec2::ZERO));

        app.add_systems(OnEnter(Screen::Loading), load_assets);
        app.add_systems(Startup, (spawn_camera, load_assets));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Default Camera"), DefaultCamera, Camera2d));
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    // Load fonts
    // NOTE: These may be different later, for now they're the same.
    let ui_font = asset_server.load("fonts/Pixellari.ttf");
    let game_font = asset_server.load("fonts/Pixellari.ttf");

    // Load images
    // NOTE: These may be different later, for now they're the same.
    let shopper_npc = asset_server.load("shopper/shopper.png");
    let shopper_player = asset_server.load("shopper/shopper.png");

    commands.insert_resource(GameAssets {
        ui_font,
        game_font,
        shopper_npc,
        shopper_player,
    });
    next_screen.set(Screen::MainMenu);
}
