use bevy::prelude::*;

use crate::data::{
    colors::{BACKGROUND_COLOR, CREDITS_FONT_COLOR},
    constants::*,
};

pub fn background_plugin(app: &mut App) {
    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, spawn_credits);
}

fn spawn_credits(mut commands: Commands) {
    commands.spawn((
        Text::new("Made by: @Cova-ops"),
        TextFont {
            font_size: CREDITS_FONT_SIZE,
            ..default()
        },
        TextColor(CREDITS_FONT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            bottom: CREDITS_TEXT_BOTTOM_PADDING,
            right: CREDITS_TEXT_RIGHT_PADDING,
            ..default()
        },
    ));
}
