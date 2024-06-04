use std::ops::Neg;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // First phase: Detect collisions.
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    // Second phase: Update colliders.
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

//returns the velocity of two bodys with mass m1/m2 and velocity v1/v2 after elastic, CENTRIC collision, calculated from conservation of energy and momentum
pub fn calc_velocity_centric_collision(v1: Vec3, v2: Vec3, m1: f32, m2: f32) -> [Vec3; 2]{
    let v1_prime:Vec3 = (2_f32 * (m2 * v2) + m1 * v1 - m2 * v1) / (m1 + m2);
    let v2_prime:Vec3 = v1 + v1_prime - v2;
    [v1_prime, v2_prime]
}


//returns the velocity of two bodys with mass m1/m2 and velocity v1/v2 after elastic, ECCENTRIC  head-on collision, calculated from conservation of energy and momentum
pub fn calc_velocity_eccentric_collision(pos1: Vec3, pos2: Vec3, v1: Vec3, v2: Vec3, m1: f32, m2: f32) -> [Vec3; 2]{
    let a_to_b: Vec3 = pos2 - pos1;
    let dist: f32 = a_to_b.length();
    
    //part of v1 pointing towards body 2
    let v1_to_2:Vec3 = v1 * (Vec3::dot(v1, a_to_b) / v1.length() * dist);
    //part of v1 normal to v1_to_2
    let v1_normal: Vec3 =  v1 - v1_to_2;

    //part of v2 pointing towards body 1
    let v2_to_1:Vec3 = v2 * (Vec3::dot(v2, a_to_b.neg()) / v2.length() * dist);
    //part of v1 normal to v2_to_1
    let v2_normal: Vec3 =  v2 - v2_to_1;
    
    let partial_forces_after_collision: [Vec3; 2] = calc_velocity_centric_collision(v1_to_2, v2_to_1, m1, m2);

    [v1_normal + partial_forces_after_collision[0], v2_normal + partial_forces_after_collision[1]]
} 

//calculates the position to which planet b should be moved back along his velocity vector to no longer collide with planet a
pub fn calc_position_outside_collider(pos_a: Vec3, pos_b: Vec3,r_a: f32, r_b: f32, velocity_b: Vec3) -> Vec3{
    let p:f32 = (2. * (velocity_b.x * (pos_b.x - pos_a.x ) + velocity_b.y * (pos_b.y - pos_a.y))) / (velocity_b.x.powi(2) + velocity_b.y.powi(2));
    let q:f32 = (pos_b.x.powi(2) - 2. * pos_b.x * pos_a.x + pos_b.y.powi(2) - 2. * pos_b.y * pos_a.y - (r_a - r_b).powi(2)) / (velocity_b.x.powi(2) + velocity_b.y.powi(2));
    let n = - p / 2. - ((p / 2.).powi(2) - q).sqrt() - 0.0001; 
    pos_b + n * velocity_b
}
