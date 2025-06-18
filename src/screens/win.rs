use crate::{GameAssets, screens::Screen};
use bevy::{color::palettes::css::*, prelude::*};

#[derive(Component)]
struct WinScreenUI;

#[derive(Component)]
struct PlayAgainButton;

pub fn plugin(app: &mut App) {
    // Menu systems
    app.add_systems(OnEnter(Screen::Win), spawn_win_screen);
    app.add_systems(OnExit(Screen::Win), despawn_win_screen);
    app.add_systems(
        Update,
        play_again_button_interaction.run_if(in_state(Screen::Win)),
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

fn spawn_win_screen(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("Win Screen UI"),
        WinScreenUI,
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
                Text::new("Congratulations!"),
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
                Name::new("Subtitle"),
                Text::new("You acquired all the supplies you needed"),
                TextColor(GHOST_WHITE.into()),
                TextFont {
                    font: assets.ui_font.clone(),
                    font_size: 24.0,
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
                children![(
                    PlayAgainButton,
                    button("Play Again", assets.ui_font.clone(), 32.0)
                ),]
            )
        ],
    ));
}

fn despawn_win_screen(mut commands: Commands, query: Single<Entity, With<WinScreenUI>>) {
    commands.entity(query.entity()).despawn();
}

fn play_again_button_interaction(
    mut next_screen: ResMut<NextState<Screen>>,
    mut query: Single<
        (&Interaction, &mut TextColor),
        (Changed<Interaction>, With<PlayAgainButton>),
    >,
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
