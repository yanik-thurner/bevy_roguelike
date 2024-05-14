use crate::prelude::*;

#[derive(Component)]
pub struct HpText;

#[derive(Component)]
pub struct HpBar;

#[derive(Component)]
pub struct HpRoot;

fn setup_health_bar(commands: &mut Commands) {
    let hp_root = commands.spawn((HpRoot,
                                  NodeBundle {
                                      background_color: BackgroundColor(Color::rgb_linear(0.3, 0.0, 0.0)),
                                      z_index: ZIndex::Global(i32::MAX),
                                      style: Style {
                                          position_type: PositionType::Absolute,
                                          left: Val::ZERO,
                                          top: Val::ZERO,
                                          bottom: Val::Auto,
                                          right: Val::ZERO,
                                          padding: UiRect::all(Val::Px(4.0)),
                                          height: Val::Px(20.0),
                                          justify_content: JustifyContent::Center,
                                          ..default()
                                      },
                                      ..default()
                                  }
    )
    ).id();

    let hp_bar = commands.spawn((HpBar,
                                 NodeBundle {
                                     background_color: BackgroundColor(Color::rgb_linear(1.0, 0.0, 0.0)),
                                     style: Style {
                                         position_type: PositionType::Absolute,
                                         left: Val::ZERO,
                                         top: Val::ZERO,
                                         bottom: Val::Auto,
                                         right: Val::Auto,
                                         padding: UiRect::all(Val::Px(4.0)),
                                         height: Val::Px(20.0),
                                         width: Val::Percent(100.0),
                                         ..default()
                                     },
                                     ..default()
                                 }
    )
    ).id();

    let style = TextStyle {
        font_size: 16.0,
        color: Color::WHITE,
        ..default()
    };

    let hp_text = commands.spawn((HpText, TextBundle {
        text: Text::from_sections([
            TextSection {
                value: "0".into(),
                style: style.clone(),
            },
            TextSection {
                value: "/".into(),
                style: style.clone(),
            },
            TextSection {
                value: "0".into(),
                style: style.clone(),
            }
        ]),
        style: Style {
            align_self: AlignSelf::Center,
            ..default()
        },
        ..default()
    })).id();

    commands.entity(hp_root).push_children(&[hp_bar]);
    commands.entity(hp_root).push_children(&[hp_text]);
}

pub fn update_healthbar(mut commands: Commands, player_health_query: Query<&Health, With<Player>>, mut hp_bar_query: Query<&mut Style, With<HpBar>>, mut hp_text_query: Query<&mut Text, With<HpText>>) {
    let health = player_health_query.get_single().unwrap();
    let mut hp_bar = hp_bar_query.get_single_mut().unwrap();
    let mut hp_text = hp_text_query.get_single_mut().unwrap();

    hp_bar.width = Val::Percent(health.current as f32 / health.max as f32 * 100.0);
    hp_text.sections[0].value = health.current.to_string();
    hp_text.sections[2].value = health.max.to_string();
}

pub fn setup_hud(mut commands: Commands) {
    setup_health_bar(&mut commands);
}