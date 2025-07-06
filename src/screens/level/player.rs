use crate::{
    DefaultCamera, GameAssets,
    screens::{
        Screen,
        level::{GameLayer, Inventory, Item, Objectives, shelf::Shelf},
    },
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

#[derive(Component)]
struct PlayerCamera {
    speed_factor: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub current_shelf: Option<Entity>,
}

#[derive(Event)]
pub struct PlayerPickedItem;

#[derive(Component)]
struct InventoryUI;

#[derive(Component)]
struct InventoryUIContainer;

pub fn plugin(app: &mut App) {
    // Register necessary types
    app.register_type::<Player>();
    app.add_input_context::<PlayerInputContext>();

    // Player systems
    app.add_systems(
        OnEnter(Screen::Level),
        (spawn_player_camera, spawn_player, spawn_inventory_ui),
    );
    app.add_systems(
        OnExit(Screen::Level),
        (despawn_player_camera, despawn_player, despawn_inventory_ui),
    );
    app.add_systems(Update, (camera_follow, inventory_changed));

    // Player input reactions
    app.add_observer(player_acceleration);
    app.add_observer(player_steering);
    app.add_observer(player_interaction);
}

fn spawn_player_camera(
    mut commands: Commands,
    mut default_camera_query: Single<&mut Camera, With<DefaultCamera>>,
) {
    commands.spawn((
        Name::new("Player Camera"),
        PlayerCamera { speed_factor: 1.2 },
        Camera2d,
        Camera {
            is_active: true,
            ..Default::default()
        },
        RigidBody::Kinematic,
    ));
    default_camera_query.is_active = false;
}

fn despawn_player_camera(
    mut commands: Commands,
    player_camera_query: Single<Entity, With<PlayerCamera>>,
    mut default_camera_query: Single<&mut Camera, With<DefaultCamera>>,
) {
    commands.entity(player_camera_query.entity()).despawn();
    default_camera_query.is_active = true;
}

fn camera_follow(
    mut camera_query: Single<(&Transform, &PlayerCamera, &mut LinearVelocity)>,
    player_query: Single<&Transform, With<Player>>,
) {
    camera_query.2.0 = (player_query.translation - camera_query.0.translation).truncate()
        * camera_query.1.speed_factor;
}

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
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
        .to((KeyCode::Space, GamepadButton::South))
        .with_conditions(Press::default());

    // Spawn player
    let player_size = Vec2::new(72.0, 36.0);
    commands.spawn((
        Name::new("Player"),
        Player {
            current_shelf: None,
        },
        Inventory::default(),
        // Sprite::from_color(LIMEGREEN, player_size),
        Sprite {
            image: assets.shopper_player.clone(),
            custom_size: Some(Vec2::new(78.0, 78.0)),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(player_size.x, player_size.y),
        CollisionLayers::new(GameLayer::Player, [GameLayer::NPC, GameLayer::Environment]),
        LinearDamping(1.2),
        AngularDamping(2.0),
        actions,
    ));
}

fn despawn_player(mut commands: Commands, query: Single<Entity, With<Player>>) {
    commands.entity(query.entity()).despawn();
}

fn spawn_inventory_ui(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("Inventory UI"),
        InventoryUI,
        BackgroundColor(BLACK.with_alpha(0.6).into()),
        BorderRadius::new(Val::Px(8.0), Val::ZERO, Val::ZERO, Val::Px(8.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(25.0),
            right: Val::ZERO,
            width: Val::Vw(25.0),
            height: Val::Vh(50.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(4.0)),
            ..Default::default()
        },
        children![
            (
                Name::new("Inventory UI Title"),
                Text::new("Inventory"),
                TextColor(WHITE.into()),
                TextFont {
                    font: assets.ui_font.clone(),
                    font_size: 24.0,
                    ..Default::default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    width: Val::Percent(100.0),
                    ..Default::default()
                }
            ),
            (
                InventoryUIContainer,
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                }
            )
        ],
    ));
}

fn despawn_inventory_ui(mut commands: Commands, query: Single<Entity, With<InventoryUI>>) {
    commands.entity(query.entity()).despawn();
}

fn inventory_item_widget(item: Item, quantity: u32, font: Handle<Font>) -> impl Bundle {
    (
        Name::new(format!("Inventory Item {} ({})", item, quantity)),
        Text::new(format!("{} ({})", item, quantity)),
        TextFont {
            font,
            font_size: 18.0,
            ..Default::default()
        },
    )
}

fn item_objective_widget(
    item: Item,
    quantity: u32,
    required_quantity: u32,
    font: Handle<Font>,
) -> impl Bundle {
    (
        Name::new(format!(
            "Inventory Item {} ({} of {})",
            item, quantity, required_quantity
        )),
        Text::new(format!("{} ({} of {})", item, quantity, required_quantity)),
        TextFont {
            font,
            font_size: 18.0,
            ..Default::default()
        },
    )
}

fn inventory_changed(
    mut commands: Commands,
    assets: Res<GameAssets>,
    objectives: Res<Objectives>,
    player_query: Single<&Inventory, (Changed<Inventory>, With<Player>)>,
    container_query: Single<Entity, With<InventoryUIContainer>>,
) {
    // Clear out existing items
    commands
        .entity(container_query.entity())
        .despawn_related::<Children>();

    // Add objective items first
    commands
        .entity(container_query.entity())
        .with_children(|parent| {
            objectives
                .items
                .iter()
                .map(|(item, required_quantity)| {
                    let quantity = player_query.0.get(item).map_or(0, |q| *q);
                    item_objective_widget(
                        *item,
                        quantity,
                        *required_quantity,
                        assets.ui_font.clone(),
                    )
                })
                .for_each(|widget| {
                    parent.spawn(widget);
                });
        });

    // Add optional items after
    commands
        .entity(container_query.entity())
        .with_children(|parent| {
            player_query
                .0
                .iter()
                .filter(|(item, _)| !objectives.items.contains_key(item))
                .map(|(item, quantity)| {
                    inventory_item_widget(*item, *quantity, assets.ui_font.clone())
                })
                .for_each(|widget| {
                    parent.spawn(widget);
                });
        });
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
    mut commands: Commands,
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
                commands.trigger(PlayerPickedItem);
            }
        }
    }
}
