use std::ops::Neg;
use bevy::math::vec3;
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
    pub value: usize,
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, SingleForceCollection::create));
    }
}

fn update_position(mut query: Query<(&mut Velocity,&Acceleration, &mut Transform)>, time: Res<Time>) {
    for (mut velocity,acceleration, mut transform) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
        transform.translation += velocity.value * time.delta_seconds();
    }
}


//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


#[derive(Debug)]
//struct for a force acting on a planet with saved "from"- and "to"-IDs
struct SingleForce {
    from: usize,
    to: usize,
    force_vector: Vec3, 
}

impl SingleForce {
    //calculates the force acting on planet1 from planet 2
    fn calculate_gravitational_force(mass1: &Mass, mass2: &Mass, planet1_transform: &Transform, planet2_transform: &Transform, id1: &ID, id2: &ID) -> SingleForce {
        let position1_to_position2 = planet2_transform.translation - planet1_transform.translation;
        let distance: f32 = position1_to_position2.length();
        if distance == 0. {
            return SingleForce {
                from: id1.value,
                to: id2.value,
                force_vector: vec3(0.,0.,0.),
            }
        }
        let gravitational_constant:f32 = 1.;
        SingleForce {
            from: id1.value,
            to: id2.value,
            force_vector: (gravitational_constant * mass1.value * mass2.value * position1_to_position2)/f32::powf(distance,3.),
        }
    }
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Debug)]
//collection of all forces
struct SingleForceCollection {
    collection: Vec<SingleForce>,
}

impl SingleForceCollection {
    
    //calculates the number of needed force combinations
    fn calculate_num_of_combinations() -> usize{
        let mut num: usize = 0;
        for i in 1..N{
            num += N-i;
        }
        num
    }

    fn create(query: Query<(&Transform,&Mass, &ID)>){
        let num_of_combinations: usize = SingleForceCollection::calculate_num_of_combinations();
        let mut loop_counter: usize = 0;
        let mut skip_counter: usize = 1;
        
        let mut collection: SingleForceCollection = SingleForceCollection {
            collection: Vec::new(),
        };
        
        for (outer_transform,outer_mass, outer_id) in query.iter(){
            for (inner_transform,inner_mass, inner_id) in query.iter().skip(skip_counter){
                collection.collection.push( SingleForce::calculate_gravitational_force(
                    outer_mass,
                    inner_mass,
                    outer_transform,
                    inner_transform,
                    outer_id,
                    inner_id
                ));
                loop_counter += 1;
            }
            skip_counter += 1;
            if loop_counter == num_of_combinations { break; }
        }
        
        //print!("collection: {:?}", collection);
        collection.calculate_total_force(&3_usize);
    }
    
    fn calculate_total_force(&self ,id: &usize){
        let mut totalforce = TotalForce{
            id: *id,
            force_vector: vec3(0.,0.,0.),
        };
        
        //adds all forces "from" the given id to totalforce.force_vector
        if *id == 1 {
            for i in 0_usize..N-1{
                totalforce.force_vector += self.collection[i].force_vector;
            }
        } else if *id==N {
            //does nothing because if id==N there is no force with this id as "from"-Force
        } 
        else{ 
            let mut start_index: usize = 0;
            for i in 1..*id{
                start_index += N-i;
            }
            for i in start_index..start_index + N - *id{
                totalforce.force_vector += self.collection[i].force_vector;
            }
        }
        
        //adds all forces "to" the given id to totalforce,force_vector
        if *id== 1{
            //does nothing because if id==1 there are no "to"-Forces with this id
        } else {
            let mut curr_index: usize = id-2;
            totalforce.force_vector += self.collection[curr_index].force_vector.neg();
            for i in 3..2 + *id{
                curr_index += N-i;
                totalforce.force_vector += self.collection[curr_index].force_vector.neg();
            }
        }
        
        print!("totalforce for 3: {:?}", totalforce);
    }
}

#[derive(Debug)]
struct TotalForce{
    id: usize,
    force_vector: Vec3,
}

#[derive(Debug)]
struct TotalForceCollection{
    collection: Vec<TotalForce>,
}

impl TotalForceCollection {
    fn create(single_force_collection: SingleForceCollection) -> TotalForceCollection{
        
    }
}