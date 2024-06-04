use bevy::math::vec3;
use bevy::prelude::*;
use bevy::reflect::List;
use bevy::utils::HashMap;
use winit::dpi::Position;
use crate::collision::{calc_position_outside_collider, calc_velocity_eccentric_collision, Collider};

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
        app.add_systems(Update, (update_position, update_velocity, update_acceleration, handle_collisions));
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

/*

fn handle_asteroid_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider)>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // Asteroid collided with another asteroid.
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // Despawn the asteroid.
            query.get_mut
            commands.entity(entity).despawn_recursive();
        }
    }
}


 */

fn handle_collisions(
    //mut query_a: Query<(Entity,&Collider,&mut Velocity, &mut Transform, &Mass)>, mut query_b: Query<(&mut Collider,&mut Velocity, &mut Transform, &Mass)>)
    mut set: ParamSet<(Query<(Entity,&Collider,&Velocity, &Transform, &Mass)>,
                       Query<(&mut Collider,&mut Velocity, &mut Transform)>)>)
{
    //V[0] = translation
    //V[1] = Velocity.value
    //V[2] = Mass.value
    //V[3] = radius

    let mut entity_map: HashMap<Entity,(Vec3,Vec3,f32,f32)> = HashMap::new();
    
    for (entity_a , collider_a, velocity_a, transform_a, mass_a) in set.p0().iter_mut() {
        entity_map.insert(entity_a, (transform_a.translation,velocity_a.value,mass_a.value,collider_a.radius));
    }
    
    let query_0 = set.p0();
    
    for hashmap_entry in entity_map.iter_mut(){
        let query_for_value = query_0.get(*hashmap_entry.0).unwrap();
        let colliding_entities = &query_for_value.1.colliding_entities;
        
        for entity  in colliding_entities {
            let a = entity_map.entry(*entity).;
            hashmap_entry.1.0 =  vec3(3.,4.,4.);
        }
        
    }
    
    
}

    /*
    for entity_b in &collider_a.colliding_entities {

    let mut query_entity_b = set.p1().get_mut(*entity_b).unwrap();
    query_entity_b.0.colliding_entities.remove(0);
    query_entity_b.0.colliding_entities.shrink_to_fit();

    query_entity_b.2.translation = calc_position_outside_collider(transform_a.translation, query_entity_b.2.translation,
                                                                  collider_a.radius, query_entity_b.0.radius,
                                                                  query_entity_b.1.value);

    let post_collision_vel = calc_velocity_eccentric_collision(transform_a.translation, query_entity_b.2.translation,
                                                               velocity_a.value, query_entity_b.1.value,
                                                               mass_a.value, query_entity_b.3.value);

    velocity_a.value = post_collision_vel[0];
    query_entity_b.1.value = post_collision_vel[1];
}

}

let a = query_entity_b.1.value;
    query_entity_b.1.value = velocity_a.value;


      1. neuen vec mit vel,pos,rad,entity / hashmap entity as key vec(v,pos,rad) as value
    2. positionen etc. berechnen und ändern danach wieder hinzufügen

    }
     */

