mod data;
mod plugins;
mod utils;

use data::constants::*;
use plugins::background::background_plugin;
use plugins::camera::camera_plugin;
use plugins::score::score_plugin;
use plugins::snake::snake_plugin;
use plugins::wall::wall_plugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resize_constraints: WindowResizeConstraints {
                    min_width: WIDTH_SIZE_WINDOW,
                    min_height: HEIGHT_SIZE_WINDOW,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(background_plugin)
        .add_plugins(wall_plugin)
        .add_plugins(camera_plugin)
        .add_plugins(snake_plugin)
        .add_plugins(score_plugin)
        .run();
}
