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
pub(crate) const N: usize = 30;
#[derive(Bundle)]

struct PlanetBundle {
    velocity: Velocity,
    mass: Mass,
    id: ID,
    model: SpriteBundle,
}

impl PlanetBundle{
    //returns the radius, from m * k = V_sphere
    fn get_planet_radius(mass: f32) -> f32{
        let k:f32 = 10.;
        f32::cbrt(3_f32 * mass * k / 4_f32 * std::f32::consts::PI)
    }

    fn get_scaling_factor(mass: f32) -> f32{
        let radius:f32 = PlanetBundle::get_planet_radius(mass);
        radius / 1000.
    }
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
            texture: asset_server.load("planet.png"),
            transform: Transform{
                translation: STARTING_TRANSLATION,
                rotation: default(),
                scale: STANDARDT_SCALE,
            },..default()
        },
    });
}

fn spawn_n_planets(mut commands: Commands, asset_server: Res<AssetServer>){
    /*
    commands.spawn(PlanetBundle{
        velocity: Velocity {
            value: vec3(0.,0.,0.),
        },
        mass: Mass {
            value: 0.001,
        },
        id: ID {
            value: 1,
        },
        model: SpriteBundle {
            texture: asset_server.load("planet.png"),
            transform: Transform{
                translation: vec3(0.,0.,0.),
                rotation: default(),
                scale: vec3(1000f32.sqrt() / 500. + 0.01,1000f32.sqrt() / 500. + 0.01,0.)
            },..default()
        },
    });

    commands.spawn(PlanetBundle{
        velocity: Velocity {
            value: vec3(0.,44.72135955,0.),
        },
        mass: Mass {
            value: 1000.,
        },
        id: ID {
            value: 2,
        },
        model: SpriteBundle {
            texture: asset_server.load("planet.png"),
            transform: Transform{
                translation: vec3(50.,0.,0.),
                rotation: default(),
                scale: vec3(1f32.sqrt() / 500. + 0.01,1f32.sqrt() / 500. + 0.01,0.)
            },..default()
        },
    });

    commands.spawn(PlanetBundle{
        velocity: Velocity {
            value: vec3(0.,0.,0.),
        },
        mass: Mass {
            value: 100000.,
        },
        id: ID {
            value: 3,
        },
        model: SpriteBundle {
            texture: asset_server.load("planet.png"),
            transform: Transform{
                translation: vec3(0.,0.,0.),
                rotation: default(),
                scale: vec3(1f32.sqrt() / 500. + 0.01,1f32.sqrt() / 500. + 0.01,0.)
            },..default()
        },
    });
     */
    
    let mut rng = rand::thread_rng();
    for i in 0..N {
        let mass : f32 = rng.gen_range(1.0..1000.);
        let scaling_factor = PlanetBundle::get_scaling_factor(mass);
        commands.spawn(PlanetBundle{
            velocity: Velocity {
                value: vec3(rng.gen_range(-1.0..10.),rng.gen_range(-1.0..10.),0.),
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
                    translation: vec3(rng.gen_range(-1000.0..1000.),rng.gen_range(-1000.0..1000.),0.),
                    rotation: ,
                    scale: vec3(scaling_factor,scaling_factor,0.)
                },..default()
            },
        });
    }
}