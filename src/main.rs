mod planet;
mod movement;
mod debug;
mod camera;
mod ui;
mod asset_loader;
mod collision;


use bevy::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

use planet::PlanetPlugin;
use movement::MovementPlugin;
use debug::DebugPlugin;
use camera::CameraPlugin;
use crate::asset_loader::AssetLoaderPlugin;
use crate::ui::UiPlugin;


fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        //custom plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlanetPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}



