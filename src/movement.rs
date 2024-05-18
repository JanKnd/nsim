use bevy::math::vec3;
use bevy::prelude::*;

const GRAVITATIONAL_CONSTANT:f32 = 1.;

#[derive(Component, Debug)]
pub struct Velocity{
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct Acceleration{
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct Mass{
    pub value: f32,
}

//used to identify planet when calculating acceleration
#[derive(Component, Debug)]
pub struct ID{
    pub value: u32,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity, update_acceleration));
    }
}

fn update_acceleration(mut query1: Query<(&mut Acceleration, &Transform, &Mass)>, query2: Query<(&Transform, &Mass)>) {
    for (mut acceleration_1, transform_1, mass_1) in query1.iter_mut() {
        acceleration_1.value = vec3(0.,0.,0.);
        for (transform_2, mass_2) in query2.iter(){
                let r: Vec3 = transform_2.translation - transform_1.translation;
                let distance: f32 = r.length();
                if distance <= 1. {continue}
                acceleration_1.value +=  (GRAVITATIONAL_CONSTANT * mass_2.value / distance.powf(3.)) * r;
        }
    }
}
fn update_velocity(mut query: Query<(& Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

fn update_position(mut query: Query<(& Velocity, &mut Transform)>, time: Res<Time>) {
    for (mut velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}