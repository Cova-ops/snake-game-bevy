use bevy::prelude::*;
use rand::Rng;

use crate::data::constants::*;

use super::score::Score;

#[derive(Component, Clone)]
pub struct BodySnake {
    direction: Vec3,
}

#[derive(Component)]
pub struct Snake {
    direction: Vec3,
    tail: Vec<Entity>,
}

#[derive(Component)]
pub struct Apple;

#[derive(Resource)]
struct MovementTimer(Timer);

#[derive(Event, Default)]
struct GetAppleEvent;

#[derive(Event, Default)]
struct GameOverEvent;

#[derive(Event, Default)]
struct NewBodySnalePartEvent;

enum SnakeMovement {
    Up,
    Down,
    Left,
    Right,
}

impl SnakeMovement {
    fn from_direction(direction: Vec3) -> Self {
        match direction {
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            } => SnakeMovement::Up,
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            } => SnakeMovement::Down,
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            } => SnakeMovement::Left,
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            } => SnakeMovement::Right,
            _ => panic!("Invalid direction"),
        }
    }

    fn to_direction(&self) -> Vec3 {
        match self {
            SnakeMovement::Up => Vec3::new(0.0, 1.0, 0.0),
            SnakeMovement::Down => Vec3::new(0.0, -1.0, 0.0),
            SnakeMovement::Left => Vec3::new(-1.0, 0.0, 0.0),
            SnakeMovement::Right => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

pub fn snake_plugin(app: &mut App) {
    app.insert_resource(MovementTimer(Timer::from_seconds(
        SNAKE_SECONDS_PER_MOVEMENT,
        TimerMode::Repeating,
    )))
    .add_event::<GetAppleEvent>()
    .add_event::<GameOverEvent>()
    .add_event::<NewBodySnalePartEvent>()
    .add_systems(Startup, spawn_game)
    .add_systems(
        Update,
        (
            snake_movement,
            snake_movement_timer,
            listener_get_apple,
            listener_game_over,
            listener_new_body_snake_part,
        ),
    );
}

fn spawn_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let initial_tails = vec![commands
        .spawn((
            Sprite {
                image: asset_server.load("sprites/snake-tail.png"),
                custom_size: Some(Vec2::new(SNAKE_SIZE, SNAKE_SIZE)),
                ..default()
            },
            Transform::from_xyz(-SNAKE_SIZE * 3.0 / 2.0, 0.0, 1.0),
            BodySnake {
                direction: SnakeMovement::Right.to_direction(),
            },
        ))
        .id()];

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/snake-head.png"),
            custom_size: Some(Vec2::new(SNAKE_SIZE, SNAKE_SIZE)),
            ..default()
        },
        Transform::from_xyz(-SNAKE_SIZE / 2.0, 0.0, 1.0),
        Snake {
            direction: SnakeMovement::Right.to_direction(),
            tail: initial_tails,
        },
    ));

    let random_cell_x_apple: i32 = rand::thread_rng().gen_range(0..CELL_X_SIZE as i32);
    let random_cell_y_apple: i32 = rand::thread_rng().gen_range(0..CELL_Y_SIZE as i32);

    let start_apple_x = WALL_LEFT + WALL_WIDTH_OFFSET + WALL_THICKNESS + HALF_APPLE_SIZE - 10.0
        + (random_cell_x_apple as f32) * APPLE_SIZE;
    let start_apple_y = WALL_BOTTOM + WALL_HEIGHT_OFFSET + WALL_THICKNESS + HALF_APPLE_SIZE - 10.0
        + (random_cell_y_apple as f32) * APPLE_SIZE;

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/apple.png"),
            custom_size: Some(Vec2::new(APPLE_SIZE, APPLE_SIZE)),
            ..default()
        },
        Transform::from_xyz(start_apple_x, start_apple_y, 0.0),
        Apple {},
    ));
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snake_query: Query<&mut Snake, With<Snake>>,
) {
    let Ok(mut snake) = snake_query.get_single_mut() else {
        return;
    };

    let mut direction: Vec3 = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction = Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction = Vec3::new(0.0, -1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction = Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction = Vec3::new(1.0, 0.0, 0.0);
    }

    if direction.length() > 0.0 {
        snake.direction = direction.normalize();
    }
}

fn snake_movement_timer(
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    mut get_apple_event_writer: EventWriter<GetAppleEvent>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    mut snake_query: Query<(&mut Transform, &Snake), With<Snake>>,
    mut body_snake_query: Query<
        (&mut Transform, &mut BodySnake, &mut Sprite),
        (With<BodySnake>, Without<Snake>),
    >,
    apple_query: Query<&Transform, (With<Apple>, Without<Snake>, Without<BodySnake>)>,
    asset_server: Res<AssetServer>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    let Ok((mut snake_transform, snake)) = snake_query.get_single_mut() else {
        return;
    };

    let Ok(apple_transform) = apple_query.get_single() else {
        return;
    };

    let mut last_tail_position = snake_transform.translation;
    let mut last_direction = snake.direction;
    snake_transform.translation += snake.direction * SNAKE_SPEED;
    snake_transform.rotation = match SnakeMovement::from_direction(snake.direction) {
        SnakeMovement::Up => Quat::from_rotation_z(std::f32::consts::PI / 2.0),
        SnakeMovement::Down => Quat::from_rotation_z(std::f32::consts::PI * 3.0 / 2.0),
        SnakeMovement::Left => Quat::from_rotation_z(std::f32::consts::PI),
        SnakeMovement::Right => Quat::from_rotation_z(0.0),
    };

    // logic to move the snake tail
    for (i, &body_entity) in snake.tail.iter().enumerate() {
        let Ok((mut body_snake_transform, mut body_snake, mut body_snake_sprite)) =
            body_snake_query.get_mut(body_entity)
        else {
            continue;
        };

        // logic to move the snake tail
        let aux_translation = body_snake_transform.translation;
        body_snake_transform.translation = last_tail_position;
        last_tail_position = aux_translation;

        // The tail is going to move to a new direction
        if body_snake.direction != last_direction {
            let aux_direction = body_snake.direction;
            body_snake.direction = last_direction;
            last_direction = aux_direction;

            let body_snake_direction = SnakeMovement::from_direction(body_snake.direction);

            let rotation_angle: f32;

            let first_right_movement_validation = body_snake.direction.x != 0.0
                && last_direction.y != 0.0
                && body_snake.direction.x == last_direction.y;

            let second_right_movement_validation = body_snake.direction.y != 0.0
                && last_direction.x != 0.0
                && body_snake.direction.y != last_direction.x;

            let third_right_movement_validation = i == snake.tail.len() - 1;

            if first_right_movement_validation
                || second_right_movement_validation
                || third_right_movement_validation
            {
                // Movement to the Right
                rotation_angle = match body_snake_direction {
                    SnakeMovement::Up => std::f32::consts::PI / 2.0,
                    SnakeMovement::Down => std::f32::consts::PI * 3.0 / 2.0,
                    SnakeMovement::Left => std::f32::consts::PI,
                    SnakeMovement::Right => 0.0,
                };
            } else {
                //  Movement to the Left
                rotation_angle = match body_snake_direction {
                    SnakeMovement::Up => std::f32::consts::PI,
                    SnakeMovement::Down => 0.0,
                    SnakeMovement::Left => std::f32::consts::PI * 3.0 / 2.0,
                    SnakeMovement::Right => std::f32::consts::PI / 2.0,
                };
            }

            body_snake_transform.rotation = Quat::from_rotation_z(rotation_angle);

            if i != snake.tail.len() - 1 {
                body_snake_sprite.image = asset_server.load("sprites/snake-corner.png");
            }
        } else if i != snake.tail.len() - 1 {
            // Snake is going to move straight
            let body_snake_direction = SnakeMovement::from_direction(body_snake.direction);
            let rotation_angle = match body_snake_direction {
                SnakeMovement::Up => std::f32::consts::PI / 2.0,
                SnakeMovement::Down => std::f32::consts::PI * 3.0 / 2.0,
                SnakeMovement::Left => std::f32::consts::PI,
                SnakeMovement::Right => 0.0,
            };
            body_snake_transform.rotation = Quat::from_rotation_z(rotation_angle);
            body_snake_sprite.image = asset_server.load("sprites/snake-body.png");
        }
    }

    // logic to get apple
    if (snake_transform.translation - apple_transform.translation).length() < SNAKE_SIZE {
        get_apple_event_writer.send_default();
    }

    // logic to check if snake is out of bounds
    if snake_transform.translation.x > SNAKE_X_MAX
        || snake_transform.translation.x < SNAKE_X_MIN
        || snake_transform.translation.y > SNAKE_Y_MAX
        || snake_transform.translation.y < SNAKE_Y_MIN
    {
        game_over_event_writer.send_default();
    }
}

fn listener_game_over(
    mut commands: Commands,
    mut event_reader: EventReader<GameOverEvent>,
    asset_server: Res<AssetServer>,
    snake_query: Query<Entity, With<Snake>>,
) {
    for _ in event_reader.read() {
        let Ok(snake_entity) = snake_query.get_single() else {
            return;
        };

        println!("Game Over");
        let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
        commands.spawn(AudioPlayer::new(sound_effect));
        commands.entity(snake_entity).despawn_recursive();
    }
}

fn listener_get_apple(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut event_reader: EventReader<GetAppleEvent>,
    asset_server: Res<AssetServer>,
    mut apple_query: Query<&mut Transform, With<Apple>>,
    mut new_body_snake_part_event_writer: EventWriter<NewBodySnalePartEvent>,
) {
    for _ in event_reader.read() {
        let Ok(mut apple_transform) = apple_query.get_single_mut() else {
            return;
        };

        **score += 1;

        let sound_effect = asset_server.load("audio/coin_000.ogg");
        commands.spawn(AudioPlayer::new(sound_effect));

        // Reubicar la manzana
        let random_cell_x_apple: i32 = rand::thread_rng().gen_range(0..CELL_X_SIZE as i32);
        let random_cell_y_apple: i32 = rand::thread_rng().gen_range(0..CELL_Y_SIZE as i32);

        let start_apple_x = WALL_LEFT + WALL_WIDTH_OFFSET + WALL_THICKNESS + HALF_APPLE_SIZE - 10.0
            + (random_cell_x_apple as f32) * APPLE_SIZE;
        let start_apple_y = WALL_BOTTOM + WALL_HEIGHT_OFFSET + WALL_THICKNESS + HALF_APPLE_SIZE
            - 10.0
            + (random_cell_y_apple as f32) * APPLE_SIZE;

        apple_transform.translation = Vec3::new(start_apple_x, start_apple_y, 0.0);
        new_body_snake_part_event_writer.send_default();
    }
}

fn listener_new_body_snake_part(
    mut commands: Commands,
    mut event_reader: EventReader<NewBodySnalePartEvent>,
    mut snake_query: Query<&mut Snake, With<Snake>>,
    mut body_snake: Query<(&Transform, &mut Sprite, &mut BodySnake)>,
    asset_server: Res<AssetServer>,
) {
    for _ in event_reader.read() {
        let Ok(mut snake) = snake_query.get_single_mut() else {
            return;
        };

        if let Some(last_tail_entity) = snake.tail.last() {
            let Ok((last_tail_transform, mut last_tail_sprite, last_tail_body)) =
                body_snake.get_mut(*last_tail_entity)
            else {
                continue;
            };

            last_tail_sprite.image = asset_server.load("sprites/snake-body.png");

            let last_position = last_tail_transform.translation;
            let last_direction = last_tail_body.direction;

            let new_position = last_position - last_direction * SNAKE_SIZE;
            let new_rotation = last_tail_transform.rotation;

            snake.tail.push(
                commands
                    .spawn((
                        Sprite {
                            image: asset_server.load("sprites/snake-tail.png"),
                            custom_size: Some(Vec2::new(SNAKE_SIZE, SNAKE_SIZE)),
                            ..default()
                        },
                        Transform {
                            rotation: new_rotation,
                            translation: new_position,
                            ..default()
                        },
                        BodySnake {
                            direction: last_direction,
                        },
                    ))
                    .id(),
            );
        }
    }
}
