use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main(){
    App::new()
        .add_systems(Startup, spawn_planet)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_planet(mut commands: Commands) {
    commands.spawn((Position{x: 0.0, y: 0.0} , Velocity{x: 1.0, y: 1.0}));
}

fn update_position(mut query: Query<(&Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}