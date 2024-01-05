mod asteroids;
mod movement;
mod spaceship;
mod debug;
mod camera;
mod asset_loader;



use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use asteroids::AsteroidPlugin;
use asset_loader::AssetLoaderPlugin;



fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .run();

}




