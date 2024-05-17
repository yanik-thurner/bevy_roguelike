use bevy::input::keyboard::{keyboard_input_system, KeyboardInput};
use bevy::text::TextLayoutInfo;
use crate::components::map::Map;
use crate::prelude::*;

#[derive(Component)]
pub struct GameOverScreen;

pub fn game_over_system(mut next_state: ResMut<NextState<TurnState>>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        next_state.set(TurnState::Init);
    }
}

pub fn game_over_screen(mut commands: Commands) {
    let text_top = commands.spawn(TextBundle {
        text: Text::from_section("Your journey is no more.", TextStyle {
            font_size: 16.0,
            color: Color::RED,
            ..default()
        }),
        style: Style {
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        ..default()
    }).id();

    let text_mid = commands.spawn(TextBundle {
        text: Text::from_section("As the monstrous jaws close around you, your journey ends in a storm of blood and shadows. The Amulet of Yala remains a phantom in the dark, and your village stands on the brink of annihilation, haunted by the whispers of your failure.", TextStyle {
            font_size: 16.0,
            color: Color::ANTIQUE_WHITE,
            ..default()
        }),
        style: Style {
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        ..default()
    }.with_text_justify(JustifyText::Center)).id();

    let text_bot = commands.spawn(TextBundle {
        text: Text::from_sections(
            [TextSection {
                value: "However, destiny favors the bold. A new hero awaits to rise from the ashes of your fall.\n".into(),
                style: TextStyle {
                    font_size: 16.0,
                    color: Color::YELLOW,
                    ..default()
                },
            },
                TextSection {
                    value: "Press 1 to ignite the spark of a new hero, ready to defy the darkness.".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::LIME_GREEN,
                        ..default()
                    },
                }]
        ),
        style: Style {
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        ..default()
    }.with_text_justify(JustifyText::Center)).id();

    commands.spawn((GameOverScreen,
                    NodeBundle {
                        background_color: BackgroundColor(Color::BLACK),
                        z_index: ZIndex::Global(i32::MAX),
                        style: Style {
                            left: Val::ZERO,
                            top: Val::ZERO,
                            bottom: Val::ZERO,
                            right: Val::ZERO,
                            padding: UiRect::all(Val::Px(20.0)),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    })).push_children(&[text_top, text_mid, text_bot]);
}