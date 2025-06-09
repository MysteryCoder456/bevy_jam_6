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
    commands
        .spawn((
            Name::new("Main Menu UI"),
            MainMenuUI,
            Node {
                height: Val::Vh(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_self: JustifySelf::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
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
            ));

            parent.spawn((PlayButton, button("Play", assets.ui_font.clone(), 32.0)));
            parent.spawn((QuitButton, button("Quit", assets.ui_font.clone(), 32.0)));
        });
}

fn despawn_main_menu(mut commands: Commands, query: Single<Entity, With<MainMenuUI>>) {
    commands.entity(query.entity()).despawn();
}

fn play_button_interaction(query: Single<&Interaction, (Changed<Interaction>, With<PlayButton>)>) {
    match *query {
        Interaction::Pressed => {
            log::info!("Starting game...");
            // TODO
        }
        Interaction::Hovered => {}
        Interaction::None => {}
    }
}

fn quit_button_interaction(
    mut events: EventWriter<AppExit>,
    query: Single<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
) {
    match *query {
        Interaction::Pressed => {
            log::info!("Exiting app...");
            events.write(AppExit::Success);
        }
        Interaction::Hovered => {}
        Interaction::None => {}
    }
}
