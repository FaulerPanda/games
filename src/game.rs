use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::enums::PongState;
use crate::functions::despawn_screen;

const PADDLE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const PADDLE_SIZE: Vec2 = Vec2::new(100.0, 30.0);
const PADDLE_SPEED: f32 = 500.0;

const BALL_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
const BALL_DIAMETER: f32 = 30.0;
const BALL_SPEED: f32 = 500.0;
const BALL_SPAWN_POINT: Vec3 = Vec3::new(0.0, 50.0, 0.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, 0.5);

const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

#[derive(Component)]
struct Paddle;
#[derive(Component)]
struct Ball;
#[derive(Component)]
struct LeftWall;
#[derive(Component)]
struct RightWall;
#[derive(Component)]
struct BottomWall;
#[derive(Component)]
struct TopWall;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);
#[derive(Component)]
struct Collider;
#[derive(Component)]
struct ExitCondition;
#[derive(Component)]
struct OnGameScreen;
#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

#[allow(unused)]
#[derive(Resource)]
struct GameSurface {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Disabled,
    Loading,
    Game,
    GameOver,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PongState::Game), game_setup)
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            .add_systems(OnEnter(GameState::Loading), pong_setup)
            .add_systems(OnEnter(GameState::GameOver), game_exit)
            .add_systems(Update, countdown.run_if(in_state(GameState::Loading)))
            .add_systems(
                Update,
                (
                    move_paddle,
                    velocity_update,
                    check_collision,
                    exit_conditions,
                )
                    .chain()
                    .run_if(in_state(GameState::Game)),
            )
            .init_state::<GameState>();
    }
}

fn game_setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    commands.insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    game_state.set(GameState::Loading);
}

fn game_exit(mut game_state: ResMut<NextState<PongState>>) {
    println!("Game over");
    game_state.set(PongState::Menu);
}

fn pong_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    let window = match windows.single() {
        Ok(window) => window,
        Err(e) => {
            eprintln!("No access to a single window: {}", e);
            return;
        }
    };
    let width = window.width() / 2.0;
    let height = window.height() / 2.0;
    let thickness = 1.0;
    commands.insert_resource(GameSurface {
        left: -width,
        right: width,
        top: height,
        bottom: -height,
    });

    //Walls
    commands.spawn((
        Sprite::from_color(WALL_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(-width, 0.0, 0.0),
            scale: Vec3::new(thickness, height * 2.0, 1.0),
            ..default()
        },
        LeftWall,
        Collider,
        OnGameScreen,
    ));
    commands.spawn((
        Sprite::from_color(WALL_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(width, 0.0, 0.0),
            scale: Vec3::new(thickness, height * 2.0, 1.0),
            ..default()
        },
        RightWall,
        Collider,
        OnGameScreen,
    ));
    commands.spawn((
        Sprite::from_color(WALL_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, height, 0.0),
            scale: Vec3::new(width * 2.0, thickness * 2.0, 1.0),
            ..default()
        },
        TopWall,
        Collider,
        OnGameScreen,
    ));
    commands.spawn((
        Sprite::from_color(WALL_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, -height, 0.0),
            scale: Vec3::new(width * 2.0, thickness * 2.0, 1.0),
            ..default()
        },
        BottomWall,
        ExitCondition,
        OnGameScreen,
    ));

    //Paddle
    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, -height + 100.0, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        Paddle,
        Collider,
        OnGameScreen,
    ));

    //Ball
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform::from_translation(BALL_SPAWN_POINT)
            .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.0)),
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
        OnGameScreen,
    ));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Game);
    }
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Single<&mut Transform, With<Paddle>>,
    screen: Res<GameSurface>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    let left_wall = screen.left + PADDLE_SIZE.x / 2.0;
    let right_wall = screen.right - PADDLE_SIZE.x / 2.0;

    let new_paddle_position = paddle.translation.x + direction * PADDLE_SPEED * time.delta_secs();
    paddle.translation.x = new_paddle_position.clamp(left_wall, right_wall);
}

fn velocity_update(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn check_collision(
    ball: Single<(&Transform, &mut Velocity), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    let (ball_transform, mut ball_velocity) = ball.into_inner();

    for collider_transform in collider_query {
        let collision = ball_collision(
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.),
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

fn exit_conditions(
    exit_query: Query<&Transform, With<ExitCondition>>,
    ball: Single<&Transform, With<Ball>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for exit_transform in exit_query {
        let collision = ball_collision(
            BoundingCircle::new(ball.translation.truncate(), BALL_DIAMETER / 2.),
            Aabb2d::new(
                exit_transform.translation.truncate(),
                exit_transform.scale.truncate() / 2.,
            ),
        );
        if let Some(collision) = collision {
            if collision == Collision::Top {
                game_state.set(GameState::GameOver);
            }
        }
    }
}

fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center);
    let offset = ball.center - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
