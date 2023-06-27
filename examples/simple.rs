use bevy::prelude::*;
use bevy_asefile::{AsepriteBundle, AsepritePlugin};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AsepritePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(AsepriteBundle {
            aseprite: asset_server.load("player.ase"),
            transform: Transform {
                scale: Vec3::splat(4.),
                translation: Vec3::new(0., 80., 0.),
                ..Default::default()
            },
            ..Default::default()
        });
}
