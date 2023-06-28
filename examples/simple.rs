use bevy::prelude::*;
use bevy_asefile::{AsepriteAtlas, AsepriteBundle, AsepritePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AsepritePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load("characters.ase"),
        aseprite_atlas: AsepriteAtlas {
            layer: Some("Man"),
            tag: Some("Walk"),
        },
        transform: Transform {
            scale: Vec3::splat(4.),
            translation: Vec3::new(-80., 80., 0.),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load("characters.ase"),
        aseprite_atlas: AsepriteAtlas {
            layer: Some("Woman"),
            tag: Some("Walk"),
        },
        transform: Transform {
            scale: Vec3::splat(4.),
            translation: Vec3::new(80., 80., 0.),
            ..Default::default()
        },
        sprite: TextureAtlasSprite {
            flip_x: true,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load("characters.ase"),
        aseprite_atlas: AsepriteAtlas {
            layer: Some("Woman::Hair"),
            tag: Some("Walk"),
        },
        transform: Transform {
            scale: Vec3::splat(4.),
            translation: Vec3::new(160., 80., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}
