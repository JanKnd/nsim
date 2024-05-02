use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;
use crate::movement::{ ID, Mass, Velocity};

const STARTING_VELOCITY: Vec3 = Vec3::new(0.,0.,0.);

const STARTING_ACCELERATION: Vec3 = Vec3::new(10.,1.,0.);
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.,0.,0.);
const STANDARDT_SCALE: Vec3 = Vec3::new(0.05,0.05,1.);
const STANDARDT_MASS: f32 = 100.;

//number of planets
pub(crate) const N: usize = 10;
#[derive(Bundle)]

struct PlanetBundle {
    velocity: Velocity,
    mass: Mass,
    id: ID,
    model: SpriteBundle,
}
pub struct PlanetPlugin;

impl Plugin for PlanetPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_n_planets);
    }
}


fn spawn_planet(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(PlanetBundle{
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        mass: Mass {
            value: STANDARDT_MASS,
        },
        id: ID {
            value: 1,
        },
        model: SpriteBundle {
            texture: asset_server.load("Frame 1.png"),
            transform: Transform{
                translation: STARTING_TRANSLATION,
                rotation: default(),
                scale: STANDARDT_SCALE,
            },..default()
        },
    });
}

fn spawn_n_planets(mut commands: Commands, asset_server: Res<AssetServer>){
    let mut rng = rand::thread_rng();
    for i in 0..N {
        let mass : f32 = rng.gen_range(1.0..1000.);
        commands.spawn(PlanetBundle{
            velocity: Velocity {
                value: vec3(rng.gen_range(-1.0..1.),rng.gen_range(-1.0..1.),0.),
            },
            mass: Mass {
                value: mass,
            },
            id: ID {
                value: i+1,
            },
            model: SpriteBundle {
                texture: asset_server.load("Frame 1.png"),
                transform: Transform{
                    translation: vec3(rng.gen_range(-100.0..100.),rng.gen_range(-100.0..100.),0.),
                    rotation: default(),
                    scale: vec3(mass.sqrt() / 300. + 0.01,mass.sqrt() / 300. + 0.01,0.)
                },..default()
            },
        });
    }
}