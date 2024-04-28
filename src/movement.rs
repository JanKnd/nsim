use bevy::prelude::*;


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
        app.add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&mut Velocity,&Acceleration, &mut Transform)>, time: Res<Time>) {
    for (mut velocity,acceleration, mut transform) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
        transform.translation += velocity.value * time.delta_seconds();
    }
}