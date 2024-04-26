use bevy::prelude::*;
use crate::movement::Velocity;


pub struct PlanetPlugin;

impl Plugin for PlanetPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planet);
    }
}


fn spawn_planet(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(),
    Velocity {
        value: Vec3::new(0., 0., 0.),
    }
    ));
}
