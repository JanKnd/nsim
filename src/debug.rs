use bevy::prelude::*;
use crate::movement::ID;

pub struct DebugPlugin;

impl Plugin for DebugPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

fn print_position(query: Query<(Entity, &Transform, &ID)>) {
    for (entity, transform, id) in query.iter() {
        info!("Entity {:?} with ID {:?} is at Position {:?}", entity, id, transform.translation)
    }
}
