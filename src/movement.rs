use bevy::prelude::*;
use crate::planet::*;

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







//struct for a force acting on a planet with saved "from"- and "to"-IDs
struct Force{
    from: u32,
    to: u32,
    force_vector: Vec3, 
}

impl Force {
    //calculates the force acting on planet1 from planet 2
    fn calculate_gravitational_force(mass1: &Mass, mass2: &Mass, planet1_transform: &Transform, planet2_transform: &Transform, id1: &u32, id2: &u32) -> Force{
        let position1_to_position2 = planet2_transform.translation - planet1_transform.translation;
        let gravitational_constant:f32 = 1.;
        Force{
            from: *id1,
            to: *id2,
            force_vector: (gravitational_constant * mass1.value * mass2.value * position1_to_position2)/f32::powf(position1_to_position2.length(),3.),
        }
    }
}

//collection of all forces
struct ForceCollection{
    collection: Vec<Force>,
}

impl ForceCollection {
    
    //calculates the number of needed force combinations
    fn calculate_num_of_combinations() -> u32{
        let mut num: u32 = 0;
        for i in 1..N{
            num += N-i;
        }
        num
    }

    
}