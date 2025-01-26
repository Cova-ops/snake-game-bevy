use bevy::prelude::*;

use crate::data::colors::WALL_COLOR;
use crate::data::constants::*;

// Default must be implemented to define this as a required component for the Wall component below
#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
#[require(Sprite, Transform, Collider)]
pub struct Wall;

enum WallLocation {
    Top,
    Bottom,
    Left,
    Right,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Top => Vec2::new(WALL_WIDTH_OFFSET, WALL_TOP + WALL_HEIGHT_OFFSET),
            WallLocation::Bottom => Vec2::new(WALL_WIDTH_OFFSET, WALL_BOTTOM + WALL_HEIGHT_OFFSET),
            WallLocation::Left => Vec2::new(WALL_LEFT + WALL_WIDTH_OFFSET, WALL_HEIGHT_OFFSET),
            WallLocation::Right => Vec2::new(WALL_RIGHT + WALL_WIDTH_OFFSET, WALL_HEIGHT_OFFSET),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = WALL_TOP - WALL_BOTTOM;
        let arena_width = WALL_RIGHT - WALL_LEFT;

        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Top | WallLocation::Bottom => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
        }
    }
}

impl Wall {
    fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
        (
            Wall,
            Sprite::from_color(WALL_COLOR, Vec2::ONE),
            Transform {
                translation: location.position().extend(0.0),
                scale: location.size().extend(1.0),
                ..default()
            },
        )
    }
}

fn init_walls(mut commands: Commands) {
    commands.spawn(Wall::new(WallLocation::Top));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
}

pub fn wall_plugin(app: &mut App) {
    app.add_systems(Startup, init_walls);
}
