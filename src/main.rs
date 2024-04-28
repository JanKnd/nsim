mod planet;
mod movement;
mod debug;
mod camera;
mod ui;

use bevy::prelude::*;
use planet::PlanetPlugin;
use movement::MovementPlugin;
use debug::DebugPlugin;
use camera::CameraPlugin;
use crate::ui::UiPlugin;


fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        //custom plugins
        .add_plugins(UiPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlanetPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}



