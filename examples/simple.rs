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

    /* layer group */
    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load("characters.ase"),
        aseprite_atlas: AsepriteAtlas {
            layer: Some("Man"),
            tag: Some("Walk"),
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::splat(4.),
            translation: Vec3::new(-80., 80., 0.),
            ..Default::default()
        },
        ..Default::default()
    });

    /* layer */
    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load("characters.ase"),
        aseprite_atlas: AsepriteAtlas {
            layer: Some("Woman::Hair"),
            tag: Some("Walk"),
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::splat(4.),
            translation: Vec3::new(-200., 80., 0.),
            ..Default::default()
        },
        ..Default::default()
    });

    /* slice */
    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load("ui.aseprite"),
        aseprite_atlas: AsepriteAtlas {
            slice: Some("SpeechBubble"),
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::splat(4.),
            translation: Vec3::new(120., 80., 0.),
            ..Default::default()
        },
        ..Default::default()
    });

    /* slice ninepatch */
    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load("ui.aseprite"),
        aseprite_atlas: AsepriteAtlas {
            slice: Some("SpeechBubble"),
            ninepatch: Some(1),
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::splat(4.),
            translation: Vec3::new(240., 80., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}
