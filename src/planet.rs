use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;
use crate::asset_loader::SceneAssets;
use crate::collision::Collider;
use crate::movement::{Acceleration, ID, Mass, Velocity};

const GRAVITATIONAL_CONSTANT: f32 = 10.;
const STARTING_VELOCITY: Vec3 = Vec3::new(0.,0.,0.);

const STARTING_ACCELERATION: Vec3 = Vec3::new(0.,0.,0.);
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.,0.,0.);
const STANDARDT_SCALE: Vec3 = Vec3::new(0.05,0.05,1.);
const STANDARDT_MASS: f32 = 10.;

//number of planets
const N: u32 = 3;

#[derive(Bundle)]

struct PlanetBundle {
    velocity: Velocity,
    acceleration: Acceleration,
    mass: Mass,
    collider: Collider,
    model: SpriteBundle,
}

impl PlanetBundle {
    fn get_scale_vec(mass: &f32) -> Vec3{
        let scaling_factor:f32 = 2. * PlanetBundle::get_planet_radius(mass) / 1024_f32;
        vec3(scaling_factor, scaling_factor, 0.)
    }

    fn get_planet_radius(mass: &f32) -> f32{
       let proportionalitätsfaktor: f32 = 10.;
        f32::cbrt(3f32 * mass * proportionalitätsfaktor / 4f32 * std::f32::consts::PI)
    }
}
pub struct PlanetPlugin;

impl Plugin for PlanetPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_n_planets);
    }
}

/*
fn spawn_planet(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(PlanetBundle{
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        acceleration: Acceleration{
          value: STARTING_ACCELERATION,  
        },
        mass: Mass {
            value: STANDARDT_MASS,
        },
        id: ID {
            value: 1,
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

 */

fn spawn_n_planets(mut commands: Commands, scene_assets: Res<SceneAssets>){
    let mut rng = rand::thread_rng();

    for i in 0..N {
        let mass:f32 = rng.gen_range(1_f32..100_f32);
        commands.spawn(PlanetBundle{
            velocity: Velocity {
                value: Vec3::new(0.,0.,0.),
            },
            acceleration: Acceleration{
                value: STARTING_ACCELERATION,
            },
            mass: Mass {
                value: mass,
            },
            collider: Collider::new(PlanetBundle::get_planet_radius(&mass)),

            model: SpriteBundle {
                texture: scene_assets.planet.clone(),
                transform: Transform{
                    translation: Vec3::new(rng.gen_range(-500.0..500.0),rng.gen_range(-500.0..500.0),0.),
                    rotation: default(),
                    scale: PlanetBundle::get_scale_vec(&mass),
                },..default()
            },
        });
    }
}