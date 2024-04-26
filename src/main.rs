mod planet;
mod movement;
mod debug;

use bevy::prelude::*;
use planet::PlanetPlugin;
use movement::MovementPlugin;
use debug::DebugPlugin;


fn main(){
    App::new()
        .add_plugins(PlanetPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}



