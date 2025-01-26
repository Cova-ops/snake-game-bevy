use bevy::prelude::*;

use crate::{data::colors::SCOREBOARD_TEXT_COLOR, data::constants::*};

#[derive(Resource, Deref, DerefMut)]
pub struct Score(usize);

#[derive(Component)]
struct ScoreboardUi;

pub fn score_plugin(app: &mut App) {
    app.insert_resource(Score(0))
        .add_systems(Startup, spawn_scoreboard)
        .add_systems(Update, update_scoreboard);
}

fn spawn_scoreboard(mut commands: Commands) {
    commands
        .spawn((
            Text::new("Score: "),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCOREBOARD_TEXT_COLOR),
            ScoreboardUi,
            Node {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_TOP_PADDING,
                left: SCOREBOARD_TEXT_LEFT_PADDING,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCOREBOARD_TEXT_COLOR),
        ));
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}
