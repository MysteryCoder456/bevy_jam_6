use crate::{GameAssets, screens::Screen};
use bevy::{color::palettes::css::*, prelude::*};

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitButton;

pub fn plugin(app: &mut App) {
    // Menu systems
    app.add_systems(OnEnter(Screen::MainMenu), spawn_main_menu);
    app.add_systems(OnExit(Screen::MainMenu), despawn_main_menu);
    app.add_systems(
        Update,
        (play_button_interaction, quit_button_interaction).run_if(in_state(Screen::MainMenu)),
    );
}

fn button(label: impl Into<String>, font: Handle<Font>, size: f32) -> impl Bundle {
    let label_str = label.into();

    (
        Name::new(format!("{} Button", &label_str)),
        Button,
        Text::new(&label_str),
        TextFont {
            font: font,
            font_size: size,
            ..Default::default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..Default::default()
        },
    )
}

fn spawn_main_menu(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("Main Menu UI"),
        MainMenuUI,
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        children![
            (
                Name::new("Title"),
                Text::new("Toilet Paper"),
                TextColor(WHITE.into()),
                TextFont {
                    font: assets.ui_font.clone(),
                    font_size: 48.0,
                    ..Default::default()
                },
                TextLayout {
                    justify: JustifyText::Center,
                    ..Default::default()
                },
            ),
            (
                Name::new("Button Container"),
                Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::vertical(Val::Px(32.0)),
                    row_gap: Val::Px(12.0),
                    ..Default::default()
                },
                children![
                    (PlayButton, button("Play", assets.ui_font.clone(), 32.0)),
                    (QuitButton, button("Quit", assets.ui_font.clone(), 32.0))
                ]
            )
        ],
    ));
}

fn despawn_main_menu(mut commands: Commands, query: Single<Entity, With<MainMenuUI>>) {
    commands.entity(query.entity()).despawn();
}

fn play_button_interaction(
    mut next_screen: ResMut<NextState<Screen>>,
    mut query: Single<(&Interaction, &mut TextColor), (Changed<Interaction>, With<PlayButton>)>,
) {
    match *query.0 {
        Interaction::Pressed => {
            log::info!("Starting game...");
            next_screen.set(Screen::Level);
        }
        Interaction::Hovered => {
            *query.1 = SLATE_GRAY.into();
        }
        Interaction::None => {
            *query.1 = WHITE.into();
        }
    }
}

fn quit_button_interaction(
    mut events: EventWriter<AppExit>,
    mut query: Single<(&Interaction, &mut TextColor), (Changed<Interaction>, With<QuitButton>)>,
) {
    match *query.0 {
        Interaction::Pressed => {
            log::info!("Exiting app...");
            events.write(AppExit::Success);
        }
        Interaction::Hovered => {
            *query.1 = SLATE_GRAY.into();
        }
        Interaction::None => {
            *query.1 = WHITE.into();
        }
    }
}
