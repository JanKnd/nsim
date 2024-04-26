

use bevy::prelude::*;
use crate::movement::Velocity;

const STARTING_VELOCITY: Vec3 = Vec3::new(10.,10.,0.);
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.,0.,0.);
const STANDARDT_SCALE: Vec3 = Vec3::new(0.1,0.1,1.);

#[derive(Bundle)]

struct PlanetBundle {
    velocity: Velocity,
    model: SpriteBundle,
}
pub struct PlanetPlugin;

impl Plugin for PlanetPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planet);
    }
}

/*
fn spawn_planet(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(),
    Velocity {
        value: Vec3::new(0., 0., 0.),
    }
    ));
}
*/



fn spawn_planet(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(PlanetBundle{
        velocity: Velocity{
            value: STARTING_VELOCITY,
        },
        model: SpriteBundle {
            texture: asset_server.load("planet.png"),
            transform: Transform{
                translation: STARTING_TRANSLATION,
                rotation: default(),
                scale: STANDARDT_SCALE,
            },..default()
        },
    });
}