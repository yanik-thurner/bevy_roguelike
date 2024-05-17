use bevy::input::keyboard::{keyboard_input_system, KeyboardInput};
use bevy::text::TextLayoutInfo;
use crate::components::map::Map;
use crate::prelude::*;

#[derive(Component)]
pub struct GameEndScreen;

pub fn game_end_system(mut next_state: ResMut<NextState<TurnState>>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        next_state.set(TurnState::Init);
    }
}

fn game_end_screen(commands: &mut Commands, string_top: &str, string_mid: &str, string_bot1: &str, string_bot2: &str, victory: bool) {
    let text_top = commands.spawn(TextBundle {
        text: Text::from_section(string_top, TextStyle {
            font_size: 16.0,
            color: if victory { Color::LIME_GREEN } else { Color::RED },
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
        text: Text::from_section(
            string_mid,
            TextStyle {
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
                value: format!("{}\n", string_bot1),
                style: TextStyle {
                    font_size: 16.0,
                    color: Color::YELLOW,
                    ..default()
                },
            },
                TextSection {
                    value: string_bot2.into(),
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

    commands.spawn((GameEndScreen,
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

pub fn victory_screen(mut commands: Commands) {
    game_end_screen(&mut commands,
                    "Light triumphs over darkness.",
                    "With the beast's roar silenced and the Amulet of Yala reclaimed, your hero stands as a living legend, a symbol of courage and resilience. Your village celebrates beneath the sun's golden embrace, its wounds healed by the touch of victory.",
                    "But the journey is far from over.",
                    "Press 1 to embrace the next chapter of your hero's epic tale, for the world awaits new champions to rise.",
                    true,
    );
}

pub fn game_over_screen(mut commands: Commands) {
    game_end_screen(&mut commands,
                    "Your journey is no more.",
                    "As the monstrous jaws close around you, your journey ends in a storm of blood and shadows. The Amulet of Yala remains a phantom in the dark, and your village stands on the brink of annihilation, haunted by the whispers of your failure.",
                    "However, destiny favors the bold. A new hero awaits to rise from the ashes of your fall.",
                    "Press 1 to ignite the spark of a new hero, ready to defy the darkness.",
                    false)
}