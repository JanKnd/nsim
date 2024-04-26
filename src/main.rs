mod planet;
mod movement;
mod debug;
mod camera;

use bevy::prelude::*;
use planet::PlanetPlugin;
use movement::MovementPlugin;
use debug::DebugPlugin;
use camera::CameraPlugin;


fn main(){
    App::new()
        .add_plugins(DefaultPlugins)

        .add_plugins(CameraPlugin)
        .add_plugins(PlanetPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}



