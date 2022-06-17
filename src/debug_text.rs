use bevy::prelude::*;

use crate::{
    components::DebugTextSnakePosition,
    resources::{DebugSnakePosition, IsDebug},
    WIN_HEIGHT, WIN_WIDTH,
};

const EMPTY_STRING: String = String::new();

pub(crate) fn spawn_debug_text(
    mut commands: Commands,
    is_debug: Res<IsDebug>,
    asset_server: Res<AssetServer>,
) {
    if !**is_debug {
        return;
    }

    let style = TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 12.0,
        color: Color::WHITE,
    };
    let make_section = |value: String| TextSection {
        value,
        style: style.clone(),
    };

    let x = 4.0 - (WIN_WIDTH / 2.0);
    let y = 18.0 - (WIN_HEIGHT / 2.0);
    let alignment = Default::default();
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    make_section("(".into()),
                    make_section(EMPTY_STRING), // x value placeholder
                    make_section(", ".into()),
                    make_section(EMPTY_STRING), // y value placeholder
                    make_section(")".into()),
                ],
                alignment,
            },
            transform: Transform {
                translation: Vec3::new(x, y, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(DebugTextSnakePosition);
}

pub(crate) fn update_snake_head_position_text(
    is_debug: Res<IsDebug>,
    pos: Res<DebugSnakePosition>,
    mut texts: Query<&mut Text, With<DebugTextSnakePosition>>,
) {
    if !**is_debug {
        return;
    }

    for mut t in texts.iter_mut() {
        t.sections[1].value = pos.0.x.to_string();
        t.sections[3].value = pos.0.y.to_string();
    }
}
