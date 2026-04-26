/*
 * Bevy Tutorial: Pong
 * https://taintedcoders.com/bevy/tutorials/pong-tutorial
 */

use bevy::prelude::*;

const BALL_SIZE: f32 = 10.0;
const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_COLOUR: Color = Color::srgb(1.0, 0.0, 0.0);

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component)]
#[require(Position)]
struct Ball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera))
        .add_systems(
            FixedUpdate,
            (move_ball.before(project_positions), project_positions),
        )
        .run();
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning the ball...");

    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOUR);

    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

fn move_ball(mut ball: Query<&mut Position, With<Ball>>) {
    if let Ok(mut position) = ball.single_mut() {
        position.0.x += 1.0
    }
}

/* A camera is required to view anything. */
fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2d);
}

/* Update the position of objects within bevys engine. */
fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    /* Iterate over every entity within the game world. */
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.0);
    }
}
