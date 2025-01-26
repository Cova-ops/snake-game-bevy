use bevy::ui::Val;

// Snake
// ! IMPORTANT:
// ! This values should be multiples of
// ! (abs(WALL_TOP) + abs(WALL_BOTTOM) - WALL_THICKNESS) and (abs(WALL_LEFT) + abs(WALL_RIGHT) - WALL_THICKNESS)
pub const SNAKE_SPEED: f32 = 40.0;
pub const SNAKE_SIZE: f32 = 40.0;
pub const APPLE_SIZE: f32 = SNAKE_SIZE;

pub const SNAKE_SECONDS_PER_MOVEMENT: f32 = 0.2;

pub const HALF_SNAKE_SIZE: f32 = SNAKE_SIZE / 2.0;
pub const HALF_APPLE_SIZE: f32 = APPLE_SIZE / 2.0;

pub const SNAKE_Y_MAX: f32 =
    WALL_TOP + WALL_HEIGHT_OFFSET - WALL_THICKNESS - HALF_SNAKE_SIZE + 10.0;
pub const SNAKE_Y_MIN: f32 =
    WALL_BOTTOM + WALL_HEIGHT_OFFSET + WALL_THICKNESS + HALF_SNAKE_SIZE - 10.0;
pub const SNAKE_X_MAX: f32 =
    WALL_RIGHT + WALL_WIDTH_OFFSET - WALL_THICKNESS - HALF_SNAKE_SIZE + 10.0;
pub const SNAKE_X_MIN: f32 =
    WALL_LEFT + WALL_WIDTH_OFFSET + WALL_THICKNESS + HALF_SNAKE_SIZE - 10.0;

// Window
pub const WIDTH_SIZE_WINDOW: f32 = 800.0;
pub const HEIGHT_SIZE_WINDOW: f32 = 600.0;

// Walls
pub const WALL_HEIGHT_OFFSET: f32 = -40.0;
pub const WALL_WIDTH_OFFSET: f32 = 0.0;

pub const WALL_THICKNESS: f32 = 20.0;

pub const WALL_LEFT: f32 = -450.0;
pub const WALL_RIGHT: f32 = 450.0;
pub const WALL_TOP: f32 = 270.0;
pub const WALL_BOTTOM: f32 = -270.0;

// Cells
pub const CELL_X_SIZE: f32 =
    ((ABS_WALL_RIGHT + ABS_WALL_LEFT - WALL_THICKNESS) / SNAKE_SIZE) as i32 as f32;
pub const CELL_Y_SIZE: f32 =
    ((ABS_WALL_TOP + ABS_WALL_BOTTOM - WALL_THICKNESS) / SNAKE_SIZE) as i32 as f32;

// Aux
pub const ABS_WALL_BOTTOM: f32 = WALL_BOTTOM * (1.0 - 2.0 * ((WALL_BOTTOM < 0.0) as i32) as f32);
pub const ABS_WALL_TOP: f32 = WALL_TOP * (1.0 - 2.0 * ((WALL_TOP < 0.0) as i32) as f32);

pub const ABS_WALL_LEFT: f32 = WALL_LEFT * (1.0 - 2.0 * ((WALL_LEFT < 0.0) as i32) as f32);
pub const ABS_WALL_RIGHT: f32 = WALL_RIGHT * (1.0 - 2.0 * ((WALL_RIGHT < 0.0) as i32) as f32);

// Scoreboard
pub const SCOREBOARD_FONT_SIZE: f32 = 33.0;
pub const SCOREBOARD_TEXT_LEFT_PADDING: Val = Val::Px(180.0);
pub const SCOREBOARD_TEXT_TOP_PADDING: Val = Val::Px(40.0);

// Credits
pub const CREDITS_FONT_SIZE: f32 = 18.0;
pub const CREDITS_TEXT_RIGHT_PADDING: Val = Val::Px(180.0);
pub const CREDITS_TEXT_BOTTOM_PADDING: Val = Val::Px(20.0);
